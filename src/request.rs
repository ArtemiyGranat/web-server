use std::str::FromStr;

use crate::{header::Header, method::Method};

pub struct Request {
    method: Method,
    target: String,
    // Should I always use HTTP version equals 1.1 or parse it from request?...
    // protocol: String,
    headers: Vec<Header>,
    body: String,
}

impl Request {
    // TODO: HTTP version handling and code refactoring
    pub fn new(raw_request: &str) -> Result<Self, &str> {
        let mut lines = raw_request.lines();

        let mut request_line = lines
            .next()
            .ok_or("Invalid request")?
            .split_ascii_whitespace();

        let method = request_line
            .next()
            .ok_or("Invalid request method")?
            .to_string();

        let target = request_line
            .next()
            .ok_or("Invalid request path")?
            .to_string();
        let http_version = request_line
            .next()
            .ok_or("Invalid HTTP version")?
            .to_string();

        let mut headers = Vec::new();
        while let Some(header_line) = lines.next() {
            if header_line.trim().is_empty() {
                break;
            }

            let mut parts = header_line.split(":");
            let name = parts
                .next()
                .ok_or("Invalid header")?
                .trim()
                .to_string();
            let value = parts
                .next()
                .ok_or("Invalid header")?
                .trim()
                .to_string();
            headers.push(Header::new(name, value));
        }

        let body = lines.collect::<Vec<_>>().join("\n");
        Ok(Self {
            method: Method::from_str(&method).unwrap(),
            target,
            headers,
            body,
        })
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn headers(&self) -> &Vec<Header> {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
