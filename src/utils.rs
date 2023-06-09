pub const CRLF: &str = "\r\n";

#[cfg(not(feature = "logger"))]
pub struct DefaultLogger;

impl Default for DefaultLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "logger"))]
impl DefaultLogger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn error(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}
