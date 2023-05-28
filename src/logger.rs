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

    pub fn colored(self) -> Self {
        Self {
            level: self.level,
            colored: true,
        }
    }

    pub fn log(&self, level: LogLevel, msg: String) {
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
