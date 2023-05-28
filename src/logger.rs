#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }
}

pub struct Logger {
    level: LogLevel
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        if level >= self.level {
            println!("[{}] {}", level.as_str(), msg)
        }
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warning(&self, msg: &str) {
        self.log(LogLevel::Warning, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
}