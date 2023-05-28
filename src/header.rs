use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

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
}
