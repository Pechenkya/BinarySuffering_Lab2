use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn create_error(message: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, message))
}

pub fn read_file_binary(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn write_file_binary(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;

    file.write_all(data)?;

    Ok(())
}
