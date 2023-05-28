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
                let (status_code, target) = if file_exists(request.target()) {
                    (200, request.target())
                } else {
                    (404, "404.html")
                };

                match read_file(target) {
                    Ok(content) => Response::new(status_code, Vec::new(), content),
                    Err(_) => Response::new(500, Vec::new(), String::from("500 Internal Server Error")),
                }
            }
            Method::Unknown => {
                Response::new(400, Vec::new(), String::from("400 Bad Request"))
            }
        }
    }
}
