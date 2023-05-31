use std::fs;
use std::io;
use std::path::Path;

pub struct File {
    data: String,
}

impl File {
    pub fn new(path: &str) -> Self {
        let data = if Self::file_exists(path) {
            Self::read_file(path).unwrap()
        } else {
            // TODO: Error handling
            "".to_string()
        };
        Self { data }
    }

    pub fn read_file(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(Path::new(path))
    }

    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}

impl From<File> for String {
    fn from(file: File) -> Self {
        file.data
    }
}
