use crate::header::HttpHeader;
use crate::http_version::HttpVersion;
use crate::status_code::HttpStatusCode;
use crate::utils::CRLF;
use std::fmt::Write;

/// An HTTP response.
pub struct HttpResponse {
    status_code: HttpStatusCode,
    headers: Vec<HttpHeader>,
    body: String,
}

impl HttpResponse {
    /// Creates a new `HttpResponse` with specific HTTP status code.
    pub fn new(status_code: HttpStatusCode) -> Self {
        Self {
            status_code,
            headers: Vec::new(),
            body: format!("{}", status_code),
        }
    }

    // TODO: Add checks for forbidden headers
    fn add_header(&mut self, header: HttpHeader) {
        self.headers.push(header)
    }

    /// Constructs a `Response` with specific header.
    pub fn with_header(mut self, header: HttpHeader) -> Self {
        self.add_header(header);
        self
    }

    /// Constructs a `Response` with specific body.
    pub fn with_body<B>(mut self, body: B) -> Self
    where
        B: Into<String>,
    {
        self.body = body.into();
        self
    }

    /// Returns the response's HTTP status code.
    pub fn status_code(&self) -> &HttpStatusCode {
        &self.status_code
    }

    /// Returns a `String` representation of the HTTP response.
    pub fn to_string(&self, http_version: &HttpVersion) -> String {
        let mut response = String::new();

        write!(response, "{} {}{}", http_version, self.status_code, CRLF).unwrap();
        for header in &self.headers {
            write!(response, "{}: {}{}", header.name(), header.value(), CRLF).unwrap();
        }

        write!(response, "{}", CRLF).unwrap();
        write!(response, "{}", self.body).unwrap();

        response
    }
}
