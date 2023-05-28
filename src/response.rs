use crate::header::Header;

pub struct Response {
    status_code: u16,
    headers: Vec<Header>,
    body: String,
}

impl Response {
    pub fn new(status_code: u16, headers: Vec<Header>, body: String) -> Self {
        let mut response = Self {
            status_code,
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
        todo!()
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        todo!()
    }
}
