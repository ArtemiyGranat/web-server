#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Path {
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

        Self { parts, queries }
    }

    pub fn matches_route(&self, route: &Path) -> bool {
        let route_parts = &route.parts;
        let path_parts = &self.parts;

        if route_parts.len() != path_parts.len() {
            return false;
        }

        for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
            if !Self::matches_part(route_part, path_part) {
                return false;
            }
        }

        true
    }

    fn matches_part(route_part: &str, path_part: &str) -> bool {
        if route_part.starts_with('{') && route_part.ends_with('}') {
            true
        } else {
            route_part == path_part
        }
    }
}
