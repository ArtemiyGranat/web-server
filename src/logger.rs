use chrono::Local;

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
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    pub fn log(&self, level: LogLevel, msg: String) {
        if level >= self.level {
            let current_time = Local::now().format("%d-%m-%Y %H:%M:%S");
            println!("[{}] [{}] {}", current_time, level.as_str(), msg)
        }
    }

    pub fn debug(&self, msg: String) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn info(&self, msg: String) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warning(&self, msg: String) {
        self.log(LogLevel::Warning, msg);
    }

    pub fn error(&self, msg: String) {
        self.log(LogLevel::Error, msg);
    }
}
