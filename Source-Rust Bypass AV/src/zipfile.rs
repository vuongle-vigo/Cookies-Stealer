use std::fs::{remove_file, File};
use std::{fs, io};

use std::io::{Read, Write};
use std::path::Path;
use zip::unstable::write::FileOptionsExt;
use zip::write::FileOptions;
use zip::CompressionMethod;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn create_random_filename(length: usize) -> String {
    let mut rng = thread_rng();
    let filename: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    filename
}

pub fn zipfile(file_names: Vec<String>, zip_file_name: &str, password: &str) -> io::Result<()> {
    let mut zip_file = File::create(zip_file_name)?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .with_deprecated_encryption(password.as_bytes());

    for file_name in file_names.iter() {
        let file_content = fs::read(file_name)?;
        zip_writer.start_file(
            Path::new(file_name.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            options,
        )?;
        zip_writer.write_all(&file_content)?;
    }

    zip_writer.finish()?;

    for file_name in file_names {
        remove_file(file_name)?;
    }

    Ok(())
}
