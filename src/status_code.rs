pub struct StatusCode(u16);

impl StatusCode {
    pub fn new(code: u16) -> Self {
        Self(code)
    }

    pub fn code(&self) -> u16 {
        self.0
    }

    // TODO: Add all status code descriptions
    pub fn description(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not found",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }
}
