#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn log_error() {
        let result = log(LogLevel::Error, "Error");
        assert_eq!(result, "[ERROR]:Error")
    }

    #[test]
    fn log_warn() {
        let result = log(LogLevel::Warning, "Warning");
        assert_eq!(result, "[WARN]:Warning")
    }

    #[test]
    fn log_info() {
        let result = log(LogLevel::Info, "Info");
        assert_eq!(result, "[INFO]:Info")
    }

    #[test]
    fn test_production_rate_per_hour() {
        let result = production_rate_per_hour(4);
        assert_eq!(result, 884_f64)
    }
}
/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    String::from("[INFO]:") + message
}
pub fn warn(message: &str) -> String {
    String::from("[WARN]:") + message
}
pub fn error(message: &str) -> String {
    String::from("[ERROR]:") + message
}
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mul: f64 = 221_f64 * speed as f64;
    if speed >= 1 && speed <= 4 {
        mul * 1.0
    } else if speed <= 8 {
        mul * 0.9
    } else {
        mul * 0.8
    }
}
