pub const CRLF: &str = "\r\n";
pub const FILES_PREFIX: &str = "static/";

pub struct DefaultLogger;

impl DefaultLogger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn error(&self, msg: String) {
        eprintln!("{}", msg);
    }
}
