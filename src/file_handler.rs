use crate::utils::FILES_PREFIX;
use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(Path::new(&trim_path(path)))
}

pub fn file_exists(path: &str) -> bool {
    Path::new(&trim_path(path)).exists()
}

// TODO: Could be optimized I guess
pub fn trim_path(path: &str) -> String {
    format!("{}{}", FILES_PREFIX, path.trim_start_matches('/'))
}
