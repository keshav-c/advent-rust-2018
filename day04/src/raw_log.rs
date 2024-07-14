use once_cell::sync::Lazy;
use regex::Regex;
use time::{macros::format_description, PrimitiveDateTime};

pub struct RawLog {
    time: PrimitiveDateTime,
    message: String,
}

impl TryFrom<&str> for RawLog {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\[(?P<timestamp>.*)\] (?P<message>.*)$").unwrap());
        let caps = RE
            .captures(value)
            .ok_or(String::from("Log did not match rawlog pattern"))?;
        let message: String = String::from(caps["message"].trim());
        if message.is_empty() {
            return Err("Log message is empty".into());
        }
        let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]");
        match PrimitiveDateTime::parse(&caps["timestamp"], &time_format) {
            Ok(time) => Ok(RawLog { time, message }),
            Err(p) => Err(format!("Time Parsing Error: {}", p.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn test_rawlog_conv_str2rawlog() {
        let raw = "[1518-11-01 23:01] Guard #10 begins shift";
        let rl: RawLog = raw.try_into().unwrap();
        assert_eq!(rl.message, String::from("Guard #10 begins shift"));
        assert_eq!(1518, rl.time.year());
        assert_eq!(Month::November, rl.time.month());
        assert_eq!(1, rl.time.day());
        assert_eq!(23, rl.time.hour());
        assert_eq!(1, rl.time.minute());
    }

    #[test]
    #[should_panic(expected = "Time Parsing Error")]
    fn test_rawlog_conv_bad_timestamp() {
        let raw = "[1518-11-1 23:02] Guard #10 begins shift";
        let _rl: RawLog = raw.try_into().unwrap();
    }

    #[test]
    #[should_panic(expected = "message is empty")]
    fn test_rawlog_conv_bad_message() {
        let raw = "[1518-11-01 23:02]       ";
        let _rl: RawLog = raw.try_into().unwrap();
    }
}
