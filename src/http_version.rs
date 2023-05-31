use std::fmt;

#[derive(Clone)]
pub struct HttpVersion(u8, u8);

impl HttpVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        Self(major, minor)
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP/{}.{}", self.0, self.1) 
    }
}
