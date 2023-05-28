use crate::header::Header;
use crate::status_code::StatusCode;
use crate::utils::CRLF;
use std::fmt::Write;

pub struct Response {
    status_code: StatusCode,
    headers: Vec<Header>,
    body: String,
}

impl Response {
    pub fn new(status_code: u16, headers: Vec<Header>, body: String) -> Self {
        let mut response = Self {
            status_code: StatusCode::new(status_code),
            headers: Vec::new(),
            body,
        };

        for header in headers {
            response.add_header(header)
        }

        response
    }

    // TODO: Add checks for forbidden headers
    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header)
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response = String::new();

        write!(
            response,
            "HTTP/1.1 {} {}{}",
            self.status_code.code(),
            self.status_code.description(),
            CRLF
        )
        .unwrap();
        for header in &self.headers {
            write!(response, "{}: {}{}", header.name, header.value, CRLF).unwrap();
        }

        write!(response, "{}", CRLF).unwrap();
        write!(response, "{}", self.body).unwrap();

        response
    }
}
