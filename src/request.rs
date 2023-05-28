use crate::header::Header;

struct Request {
    method: String,
    target: String,
    protocol: String,
    headers: Vec<Header>,
    body: String,
}
