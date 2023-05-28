pub struct StatusCode(u16);

impl StatusCode {
    pub fn new(code: u16) -> Self {
        Self { 0: code }
    }

    pub fn code(&self) -> u16 {
        self.0
    }

    // TODO: Add all status code descriptions
    pub fn description(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            404 => "Not found",
            _ => "Unknown",
        }
    }
}
