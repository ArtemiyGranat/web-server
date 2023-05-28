use std::collections::HashMap;

use crate::{
    file_handler::{file_exists, read_file},
    method::Method,
    request::Request,
    response::Response,
};

pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    // TODO: Implement this
    pub fn add_route(&mut self, target: &str, handler: String) {
        todo!()
    }

    // TODO: Implement this
    pub fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::Get => {
                let response: Response;
                if file_exists(&request.target()) {
                    response = Response::new(200, Vec::new(), read_file(request.target()).unwrap());
                } else {
                    response = Response::new(404, Vec::new(), read_file("404.html").unwrap());
                }
                response
            }
            Method::Unknown => todo!(),
        }
    }
}
