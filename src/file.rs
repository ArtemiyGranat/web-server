use std::fs;
use std::io;
use std::path::Path;

/// The file handler.
pub struct File {
    data: String,
}

impl File {
    /// Creates a new `File`.
    pub fn new(path: &str) -> Self {
        let data = if Self::file_exists(path) {
            Self::read_file(path).unwrap()
        } else {
            // TODO: Error handling
            "".to_string()
        };
        Self { data }
    }

    /// Reads the file at the specified path.
    pub fn read_file(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(Path::new(path))
    }

    /// Checks if the file exists at the specified path.
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}

impl From<File> for String {
    fn from(file: File) -> Self {
        file.data
    }
}
