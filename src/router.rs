use crate::{
    method::HttpMethod, request::HttpRequest, response::HttpResponse, status_code::HttpStatusCode,
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct RouteKey {
    method: HttpMethod,
    target: String,
}

impl RouteKey {
    fn new(method: HttpMethod, target: String) -> Self {
        Self { method, target }
    }
}

type RouteMap = HashMap<RouteKey, fn(HttpRequest) -> HttpResponse>;

#[derive(Default)]
pub struct Router {
    routes: RouteMap,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(
        &mut self,
        method: HttpMethod,
        target: &str,
        handler: fn(HttpRequest) -> HttpResponse,
    ) {
        let route_key = RouteKey::new(method, target.to_string());
        self.routes.insert(route_key, handler);
    }

    // TODO: Add method not allowed or bad request or smth
    pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        match request.method() {
            HttpMethod::Get => {
                let route_key = RouteKey::new(HttpMethod::Get, request.target().to_string());
                match self.routes.get(&route_key) {
                    Some(handler) => handler(request.clone()),
                    None => HttpResponse::new(
                        // request.http_version().to_string(),
                        HttpStatusCode::NOT_FOUND,
                    ),
                }
            }
            _ => HttpResponse::new(
                // request.http_version().to_string(),
                HttpStatusCode::BAD_REQUEST,
            ),
        }
    }
}
