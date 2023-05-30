use std::str::FromStr;

// TODO: Add other methods
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Unknown,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            _ => Method::Unknown,
        }
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            _ => unreachable!(),
        }
    }
}
