use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    name: String,
    value: String,
}

// TODO: Do I really need this?
impl FromStr for Header {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Header {
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
