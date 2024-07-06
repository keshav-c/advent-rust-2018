mod fabric_box;
// use std::collections::HashMap;
use fabric_box::FabricBox;
use std::fs;

pub fn read_input(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().map(|l| l.to_owned()).collect()
}

pub fn calc_checksum(box_ids: &Vec<String>) -> i32 {
    let boxes = box_ids.into_iter().map(|id| FabricBox::from(id.to_owned()));
    let mut counts = Counts::new();
    for fb in boxes {
        if fb.twice {
            counts.twice += 1;
        }
        if fb.thrice {
            counts.thrice += 1;
        }
    }
    counts.hash()
}

struct Counts {
    twice: i32,
    thrice: i32,
}

impl Counts {
    fn new() -> Self {
        Counts {
            twice: 0,
            thrice: 0,
        }
    }

    fn hash(&self) -> i32 {
        self.twice * self.thrice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = read_input("sample.txt");
        let checksum = calc_checksum(&input);
        assert_eq!(checksum, 12)
    }
}
