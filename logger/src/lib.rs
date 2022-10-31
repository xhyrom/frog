use chrono::Local;

pub struct FrogLogger {}

pub mod colors;

impl FrogLogger {
    pub fn info(message: &str) {
        FrogLogger::log("info", colors::GREEN, message);
    }

    pub fn warn(message: &str) {
        FrogLogger::log("warn", colors::YELLOW, message);
    }

    pub fn error(message: &str) {
        FrogLogger::log("error", colors::RED, message);
    }

    pub fn log(prefix: &str, color: &str, message: &str) {
        println!("{}{}  {}{}    {}{}", colors::GRAY, Local::now().format("%d/%m/%Y %H:%M"), color, prefix, colors::RESET, message);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            $crate::FrogLogger::info(format!($($arg)*).as_str())
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            $crate::FrogLogger::warn(format!($($arg)*).as_str())
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            $crate::FrogLogger::error(format!($($arg)*).as_str())
        }
    };
}
