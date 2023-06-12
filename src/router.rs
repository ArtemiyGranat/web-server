use crate::{
    method::HttpMethod, path::Path, request::HttpRequest, response::HttpResponse,
    status_code::HttpStatusCode,
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct RouteKey {
    method: HttpMethod,
    path: Path,
}

impl RouteKey {
    fn new(method: HttpMethod, path: Path) -> Self {
        Self { method, path }
    }
}

type RouteMap = HashMap<RouteKey, fn(HttpRequest) -> HttpResponse>;

/// An HTTP router. Routes the HTTP requests to particular handlers.
#[derive(Default)]
pub struct Router {
    routes: RouteMap,
}

impl Router {
    /// Creates a new `Router`.
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Adds a new route to `Router`.
    pub fn add_route(
        &mut self,
        method: HttpMethod,
        target: &str,
        handler: fn(HttpRequest) -> HttpResponse,
    ) {
        let route_key = RouteKey::new(method, Path::new(target.to_string()));
        self.routes.insert(route_key, handler);
    }

    // TODO: Add method not allowed or bad request or smth
    /// Handles the incoming `Request` and returns `Response`.
    pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        if let Some(path) = self.find_matching_path(request.path()) {
            match request.method() {
                HttpMethod::Get => {
                    let route_key = RouteKey::new(HttpMethod::Get, path.clone());
                    if let Some(handler) = self.routes.get(&route_key) {
                        return handler(request.clone());
                    }
                    HttpResponse::new(HttpStatusCode::NOT_FOUND)
                }
                _ => HttpResponse::new(HttpStatusCode::BAD_REQUEST),
            }
        } else {
            HttpResponse::new(HttpStatusCode::BAD_REQUEST)
        }
    }

    fn find_matching_path(&self, request_path: &Path) -> Option<&Path> {
        for (route_key, _) in self.routes.iter() {
            if route_key.path.matches_route(request_path) {
                return Some(&route_key.path);
            }
        }
        None
    }
}
