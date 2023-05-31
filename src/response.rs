use crate::header::HttpHeader;
use crate::status_code::HttpStatusCode;
use crate::utils::CRLF;
use std::fmt::Write;

pub struct HttpResponse {
    // http_version: String,
    status_code: HttpStatusCode,
    headers: Vec<HttpHeader>,
    body: String,
}

impl HttpResponse {
    pub fn new<B>(status_code: HttpStatusCode, headers: Vec<HttpHeader>, body: B) -> Self
    where
        B: Into<String>,
    {
        let mut response = Self {
            // http_version,
            status_code,
            headers: Vec::new(),
            body: body.into(),
        };

        for header in headers {
            response.add_header(header)
        }
        response
    }

    // TODO: Add checks for forbidden headers
    pub fn add_header(&mut self, header: HttpHeader) {
        self.headers.push(header)
    }

    pub fn status_code(&self) -> &HttpStatusCode {
        &self.status_code
    }
}

impl ToString for HttpResponse {
    // TODO: Is there a better way to do this?
    fn to_string(&self) -> String {
        let mut response = String::new();

        write!(
            response,
            "HTTP/1.1 {}{}",
            // self.http_version,
            self.status_code,
            CRLF
        )
        .unwrap();
        for header in &self.headers {
            write!(response, "{}: {}{}", header.name(), header.value(), CRLF).unwrap();
        }

        write!(response, "{}", CRLF).unwrap();
        write!(response, "{}", self.body).unwrap();

        response
    }
}
