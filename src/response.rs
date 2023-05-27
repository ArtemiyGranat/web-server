use std::collections::HashMap;

pub struct Response {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}