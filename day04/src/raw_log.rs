use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use time::{macros::format_description, PrimitiveDateTime};

#[derive(Eq, Debug)]
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

impl Ord for RawLog {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.unix_timestamp().cmp(&other.unix_timestamp())
    }
}

impl PartialOrd for RawLog {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RawLog {
    fn eq(&self, other: &Self) -> bool {
        self.unix_timestamp() == other.unix_timestamp()
    }
}

impl RawLog {
    fn unix_timestamp(&self) -> i64 {
        self.time.assume_utc().unix_timestamp()
    }
}

pub struct RawLogs {
    logs: Vec<RawLog>,
}

impl RawLogs {
    fn new(mut logs: Vec<RawLog>) -> Self {
        logs.sort();
        RawLogs { logs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn rawlog_conv_str2rawlog() {
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
    fn rawlog_conv_bad_timestamp() {
        let raw = "[1518-11-1 23:02] Guard #10 begins shift";
        let _rl: RawLog = raw.try_into().unwrap();
    }

    #[test]
    #[should_panic(expected = "message is empty")]
    fn rawlog_conv_bad_message() {
        let raw = "[1518-11-01 23:02]       ";
        let _rl: RawLog = raw.try_into().unwrap();
    }

    #[test]
    fn rawlogs_init() {
        let (rl1, rl2, rl3, rl4) = dummy_logs();
        let input = vec![rl3, rl1, rl4, rl2];
        let collection = RawLogs::new(input);
        let (rl1, rl2, rl3, rl4) = dummy_logs();
        assert_eq!(collection.logs, vec![rl1, rl2, rl3, rl4])
    }

    fn dummy_logs() -> (RawLog, RawLog, RawLog, RawLog) {
        let rl1: RawLog = "[1518-11-01 00:00] message".try_into().unwrap();
        let rl2: RawLog = "[1518-11-01 00:05] message".try_into().unwrap();
        let rl3: RawLog = "[1518-11-01 00:25] message".try_into().unwrap();
        let rl4: RawLog = "[1518-11-01 00:30] message".try_into().unwrap();
        (rl1, rl2, rl3, rl4)
    }
}
