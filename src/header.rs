#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpHeader {
    name: String,
    value: String,
}


impl HttpHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
