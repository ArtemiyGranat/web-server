use std::collections::HashMap;

use crate::{
    file_handler::{file_exists, read_file},
    method::Method,
    request::Request,
    response::Response,
};

#[derive(PartialEq, Eq, Hash)]
pub struct RouteKey {
    method: Method,
    target: String,
}

impl RouteKey {
    pub fn new(method: Method, target: String) -> Self {
        Self { method, target }
    }
}

#[derive(Default)]
pub struct Router {
    routes: HashMap<RouteKey, fn(Request) -> Response>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    // TODO: Implement this
    fn add_route(&mut self, route_key: RouteKey, handler: fn(Request) -> Response) {
        self.routes.insert(route_key, handler);
    }

    pub fn get(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Get, target.to_string()), handler)
    }

    // TODO: Improve this method
    // Need to implement Post, Put, Delete etc handling and redirect from
    // "/" to "/index.html" maybe so it won't be 500 status code
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
                    Err(_) => {
                        Response::new(500, Vec::new(), String::from("500 Internal Server Error"))
                    }
                }
            }
            _ => Response::new(400, Vec::new(), String::from("400 Bad Request")),
        }
    }
}
