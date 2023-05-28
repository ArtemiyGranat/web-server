use crate::header::Header;

struct Request {
    method: String,
    target: String,
    // Should I use HTTP version equals 1.1 or parse it from request?...
    // protocol: String,
    headers: Vec<Header>,
    body: String,
}

impl Request {
    pub fn parse(raw_request: &str) -> Self {
        todo!()
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}