use std::collections::HashMap;

use crate::{
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

    fn add_route(&mut self, route_key: RouteKey, handler: fn(Request) -> Response) {
        self.routes.insert(route_key, handler);
    }

    pub fn get(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Get, target.to_string()), handler)
    }

    pub fn head(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Head, target.to_string()), handler)
    }

    pub fn post(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Post, target.to_string()), handler)
    }

    pub fn put(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Put, target.to_string()), handler)
    }

    pub fn delete(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Delete, target.to_string()), handler)
    }

    pub fn connect(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Connect, target.to_string()), handler)
    }

    pub fn options(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Options, target.to_string()), handler)
    }

    pub fn trace(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Trace, target.to_string()), handler)
    }

    pub fn patch(&mut self, target: &str, handler: fn(Request) -> Response) {
        self.add_route(RouteKey::new(Method::Patch, target.to_string()), handler)
    }

    // TODO: Add method not allowed or bad request or smth
    pub fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::Get => {
                let route_key = RouteKey::new(Method::Get, request.target().to_string());
                match self.routes.get(&route_key) {
                    Some(handler) => handler(request.clone()),
                    None => Response::new(404, Vec::new(), String::new()),
                }
            }
            _ => Response::new(400, Vec::new(), String::new()),
        }
    }
}
