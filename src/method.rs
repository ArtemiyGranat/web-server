// use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Unknown,
}

// impl FromStr for Method {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(match s {
//             "GET" => Method::Get,
//             "HEAD" => Method::Head,
//             "POST" => Method::Post,
//             "PUT" => Method::Put,
//             "DELETE" => Method::Delete,
//             "CONNECT" => Method::Connect,
//             "OPTIONS" => Method::Options,
//             "TRACE" => Method::Trace,
//             "PATCH" => Method::Patch,
//             _ => Method::Unknown,
//         })
//     }
// }

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "CONNECT" => Method::Connect,
            "OPTIONS" => Method::Options,
            "TRACE" => Method::Trace,
            "PATCH" => Method::Patch,
            _ => Method::Unknown,
        }
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
            _ => unreachable!(),
        }
    }
}
