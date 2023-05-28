use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(file_path);

    fs::read_to_string(path)
}
