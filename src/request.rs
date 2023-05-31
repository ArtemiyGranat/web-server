use crate::{header::Header, method::Method};

#[derive(Clone)]
pub struct Request {
    method: Method,
    target: String,
    // http_version: String,
    headers: Vec<Header>,
    body: String,
}

impl Request {
    // TODO: HTTP version handling and code refactoring
    pub fn new(raw_request: &str) -> Result<Self, &str> {
        let mut lines = raw_request.lines();

        let request_line = lines.next().ok_or("Invalid request")?;
        let (method, target, _http_version) = Self::parse_request_line(request_line)?;

        let mut headers = Vec::new();
        for header_line in lines.by_ref() {
            if header_line.trim().is_empty() {
                break;
            }

            let mut parts = header_line.split(':');
            let name = parts.next().ok_or("Invalid header")?.trim().to_string();
            let value = parts.next().ok_or("Invalid header")?.trim().to_string();
            headers.push(Header::new(name, value));
        }

        let body = lines.collect::<Vec<_>>().join("\n");
        Ok(Self {
            method,
            target,
            // http_version,
            headers,
            body,
        })
    }

    fn parse_request_line(line: &str) -> Result<(Method, String, String), &str> {
        let mut parts = line.split(' ');
        let method = parts.next().ok_or("Invalid request method")?;
        let target = parts.next().ok_or("Invalid request path")?.to_string();
        let http_version = parts.next().ok_or("Invalid HTTP version")?.to_string();

        Ok((Method::from(method), target, http_version))
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    // pub fn http_version(&self) -> &str {
    //     &self.http_version
    // }

    pub fn headers(&self) -> &Vec<Header> {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
