use std::collections::HashMap;

use crate::{method::Method, request::Request, response::Response};

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

    pub fn add_route(&mut self, method: Method, target: &str, handler: fn(Request) -> Response) {
        let route_key = RouteKey::new(method, target.to_string());
        self.routes.insert(route_key, handler);
    }

    // TODO: Add method not allowed or bad request or smth
    pub fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::Get => {
                let route_key = RouteKey::new(Method::Get, request.target().to_string());
                match self.routes.get(&route_key) {
                    Some(handler) => handler(request.clone()),
                    None => Response::new(
                        // request.http_version().to_string(),
                        404,
                        Vec::new(),
                        String::new(),
                    ),
                }
            }
            _ => Response::new(
                // request.http_version().to_string(),
                400,
                Vec::new(),
                String::new(),
            ),
        }
    }
}
