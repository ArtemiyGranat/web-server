pub const CRLF: &str = "\r\n";

#[cfg(not(feature = "logger"))]
pub struct DefaultLogger;

#[cfg(not(feature = "logger"))]
impl DefaultLogger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn error(&self, msg: String) {
        eprintln!("{}", msg);
    }
}
