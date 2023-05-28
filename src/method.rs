use std::str::FromStr;

// TODO: Add other methods
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Unknown,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Method, ()> {
        Ok(match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::Unknown,
        })
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            _ => unreachable!(),
        }
    }
}
