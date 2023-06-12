pub const CRLF: &str = "\r\n";

/// A default logger which is used if feature `logger` is disabled.
#[cfg(not(feature = "logger"))]
pub struct DefaultLogger;

// impl Default for DefaultLogger {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(not(feature = "logger"))]
impl DefaultLogger {
    /// Creates a new `DefaultLogger`.
    pub fn new() -> Self {
        Self {}
    }

    /// Logs an error.
    pub fn error(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}
