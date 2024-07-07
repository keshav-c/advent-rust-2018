mod fabric_box;
use fabric_box::FabricBox;
use std::fs;

pub fn read_input(path: &str) -> Vec<FabricBox> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().map(|id| id.into()).collect()
}

pub fn calc_checksum(boxes: &Vec<FabricBox>) -> i32 {
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

pub fn find_the_common_id(boxes: &Vec<FabricBox>) -> Result<String, ()> {
    for (i, b) in boxes.iter().enumerate() {
        if i == boxes.len() - 1 {
            return Err(());
        }
        for other_b in &boxes[(i + 1)..] {
            if let Ok(matched_str) = b.has_similar_id(other_b) {
                return Ok(matched_str);
            }
        }
    }
    Err(())
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
    fn test_sample_checksum() {
        let input = read_input("sample1.txt");
        let checksum = calc_checksum(&input);
        assert_eq!(checksum, 12)
    }

    #[test]
    fn test_sample_common_id() {
        let input = read_input("sample2.txt");
        let common_chars = find_the_common_id(&input).unwrap();
        assert_eq!(common_chars, String::from("fgij"));
    }
}
