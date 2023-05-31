use crate::{request::Request, response::Response};
use chrono::Local;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// TODO: Optimize it somehow
impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn ansi_color(&self) -> u8 {
        match self {
            LogLevel::Debug => 34,
            LogLevel::Info => 32,
            LogLevel::Warning => 33,
            LogLevel::Error => 31,
        }
    }

    pub fn as_colored_str(&self) -> String {
        format!("\x1b[{}m{}\x1b[0m", self.ansi_color(), self.as_str())
    }
}

pub struct Logger {
    level: LogLevel,
    colored: bool,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            colored: false,
        }
    }

    pub fn colored(mut self) -> Self {
        self.colored = true;
        self
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        if level >= self.level {
            let current_time = Local::now().format("%d-%m-%Y %H:%M:%S");
            let level = if self.colored {
                level.as_colored_str()
            } else {
                level.as_str().to_string()
            };
            println!("[{}] [{}] {}", current_time, level, msg)
        }
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, &msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, &msg);
    }

    pub fn warning(&self, msg: &str) {
        self.log(LogLevel::Warning, &msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, &msg);
    }

    pub fn request_received(&self, addr: &str, request: &Request) {
        let msg = format!(
            "[{}] Incoming request: {} {}",
            addr,
            request.method().as_str(),
            request.target()
        );
        self.info(&msg);
    }

    pub fn request_completed(&self, addr: &str, response: &Response) {
        let msg = format!(
            "[{}] Request completed with {} {}",
            addr,
            response.status_code().code(),
            response.status_code().description()
        );
        self.info(&msg);
    }
}
