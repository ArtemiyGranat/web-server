use std::fmt;

#[derive(Debug, Clone)]
pub struct ServerError {
    msg: String,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error: {}", self.msg)
    }
}
