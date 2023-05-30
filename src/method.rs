use std::str::FromStr;

// TODO: Add other methods
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Unknown,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Method, Self::Err> {
        Ok(match s {
            "GET" => Method::Get,
            _ => Method::Unknown,
        })
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
