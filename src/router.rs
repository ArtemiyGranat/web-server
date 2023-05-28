use std::collections::HashMap;

use crate::{request::Request, response::Response};

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
        self.routes.insert(target.to_string(), handler);
    }

    // TODO: Implement this
    pub fn handle_request(&self, request: &Request) {
        match self.routes.get(request.target()) {
            Some(target) => todo!(),
            None => todo!(),
        }
    }
}
