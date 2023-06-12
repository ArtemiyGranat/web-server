use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Path {
    path: String,
    parts: Vec<String>,
    queries: Vec<(String, String)>,
}

impl Path {
    pub fn new(path: String) -> Self {
        let parts;
        let mut queries = Vec::new();

        if let Some(query_start) = path.find('?') {
            parts = path[1..query_start]
                .split('/')
                .filter(|&part| !part.is_empty())
                .map(String::from)
                .collect();

            for query in path[query_start + 1..].split('&') {
                if let Some((key, value)) = query.split_once('=') {
                    queries.push((key.to_string(), value.to_string()));
                } else {
                    queries.push((query.to_string(), String::new()));
                }
            }
        } else {
            parts = path[1..]
                .split('/')
                .filter(|&part| !part.is_empty())
                .map(String::from)
                .collect();
        }

        Self { path, parts, queries }
    }

    pub fn matches_route(&self, route: &Path) -> bool {
        let route_parts = &self.parts;
        let request_path_parts = &route.parts;

        if request_path_parts.len() != route_parts.len() {
            return false;
        }

        for (request_path_part, route_part) in request_path_parts.iter().zip(route_parts.iter()) {
            if !Self::matches_part(route_part, request_path_part) {
                return false;
            }
        }
        true
    }

    fn matches_part(route_part: &str, path_part: &str) -> bool {
        if route_part.starts_with('{') && route_part.ends_with('}') {
            return true;
        }
        route_part == path_part
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
