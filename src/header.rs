/// The request's header.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpHeader {
    name: String,
    value: String,
}

impl HttpHeader {
    /// Creates a new `HttpHeader`.
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    /// Returns the header's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the header's value.
    pub fn value(&self) -> &str {
        &self.value
    }
}
