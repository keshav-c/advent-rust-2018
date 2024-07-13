use std::ops::Range;

use crate::fabric::Coordinates;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Claim {
    id: String,
    pub coordinates: Coordinates,
    width: i32,
    height: i32,
}

impl Claim {
    pub fn new(id: &str, x: i32, y: i32, width: i32, height: i32) -> Self {
        let coordinates = Coordinates::new(x, y);
        Self {
            id: id.to_owned(),
            coordinates,
            width,
            height,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn x_range(&self) -> Range<i32> {
        let Coordinates(x, _) = self.coordinates;
        Range {
            start: x,
            end: x + self.width,
        }
    }

    pub fn y_range(&self) -> Range<i32> {
        let Coordinates(_, y) = self.coordinates;
        Range {
            start: y,
            end: y + self.height,
        }
    }

    pub fn area(&self) -> i32 {
        self.height * self.width
    }
}

impl TryFrom<&str> for Claim {
    type Error = String;

    fn try_from(raw_claim: &str) -> Result<Self, Self::Error> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"#(?P<id>\S+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap()
        });
        if let Some(c) = RE.captures(raw_claim) {
            let id = &c["id"];
            let Ok(x) = c["x"].parse::<i32>() else {
                return Err(String::from("x could not be parsed"));
            };
            let Ok(y) = c["y"].parse::<i32>() else {
                return Err(String::from("y could not be parsed"));
            };
            let Ok(width) = c["w"].parse::<i32>() else {
                return Err(String::from("w could not be parsed"));
            };
            let Ok(height) = c["h"].parse::<i32>() else {
                return Err(String::from("h could not be parsed"));
            };
            Ok(Claim::new(&id, x, y, width, height))
        } else {
            Err(String::from("Failed to capture groups"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture1() {
        let raw = "#1 @ 1,3: 4x4";
        let claim: Claim = raw.try_into().unwrap();
        assert_eq!(claim, Claim::new("1", 1, 3, 4, 4));
    }

    #[test]
    fn test_capture2() {
        let raw = "#2 @ 3,1: 5x4";
        let claim: Claim = raw.try_into().unwrap();
        assert_eq!(claim, Claim::new("2", 3, 1, 5, 4));
    }

    #[test]
    fn test_capture3() {
        let raw = "#3 @ 5,5: 2x3";
        let claim: Claim = raw.try_into().unwrap();
        assert_eq!(claim, Claim::new("3", 5, 5, 2, 3));
    }

    #[test]
    fn test_capture4() {
        let raw = "#3 @ 5,: 2x3";
        let claim = Claim::try_from(raw);
        assert_eq!(claim, Err("Failed to capture groups".into()));
    }
}
