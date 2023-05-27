use std::collections::HashMap;

struct Request {
    method: String,
    target: String,
    protocol: String,
    headers: HashMap<String, String>,
    body: String,
}
