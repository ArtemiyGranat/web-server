use crate::{header::HttpHeader, http_version::HttpVersion, method::HttpMethod, path::Path};

/// An incoming HTTP request.
#[derive(Clone)]
pub struct HttpRequest {
    method: HttpMethod,
    path: Path,
    http_version: HttpVersion,
    headers: Vec<HttpHeader>,
    body: String,
    params: Option<Vec<String>>,
}

impl HttpRequest {
    /// Parses an incoming HTTP request from `&str`
    pub fn new(raw_request: &str) -> Result<Self, &str> {
        let mut lines = raw_request.lines();

        let request_line = lines.next().ok_or("Invalid request")?;
        let (method, path, http_version) = Self::parse_request_line(request_line)?;

        let mut headers = Vec::new();
        for header_line in lines.by_ref() {
            if header_line.trim().is_empty() {
                break;
            }

            let mut parts = header_line.split(':');
            let name = parts.next().ok_or("Invalid header")?.trim().to_string();
            let value = parts.next().ok_or("Invalid header")?.trim().to_string();
            headers.push(HttpHeader::new(name, value));
        }

        let body = lines.collect::<Vec<_>>().join("\n");
        Ok(Self {
            method,
            path,
            http_version,
            headers,
            body,
            params: None,
        })
    }

    fn parse_request_line(line: &str) -> Result<(HttpMethod, Path, HttpVersion), &str> {
        let mut parts = line.split(' ');
        let method = parts.next().ok_or("Invalid request method")?;
        let path = Path::new(parts.next().ok_or("Invalid request path")?.to_string());
        let http_version = match parts.next().ok_or("Invalid HTTP version")? {
            "HTTP/0.9" => HttpVersion::new(0, 9),
            "HTTP/1.0" => HttpVersion::new(1, 0),
            "HTTP/1.1" => HttpVersion::new(1, 1),
            "HTTP/2.0" => HttpVersion::new(2, 0),
            _ => return Err("Invalid HTTP version"),
        };

        Ok((HttpMethod::from(method), path, http_version))
    }

    /// Returns the request's method
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// Returns the request's target path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the request's HTTP version
    pub fn http_version(&self) -> &HttpVersion {
        &self.http_version
    }

    /// Returns the request's headers
    pub fn headers(&self) -> &Vec<HttpHeader> {
        &self.headers
    }

    /// Returns the request's body
    pub fn body(&self) -> &str {
        &self.body
    }
}
