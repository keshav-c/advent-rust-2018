use std::collections::HashMap;
use std::fs;

pub fn read_input(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().map(|l| l.to_owned()).collect()
}

pub fn calc_checksum(box_ids: Vec<String>) -> i32 {
    let ids = box_ids.into_iter().map(|id| Id::from(id));
    let mut counts = Counts::new();
    for id in ids {
        if id.twice {
            counts.twice += 1;
        }
        if id.thrice {
            counts.thrice += 1;
        }
    }
    counts.hash()
}

#[derive(Debug, PartialEq, Eq)]
struct Id {
    twice: bool,
    thrice: bool,
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in value.chars() {
            counts.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        Id {
            twice: has_count(&counts, 2),
            thrice: has_count(&counts, 3),
        }
    }
}

fn has_count(counts: &HashMap<char, i32>, n: i32) -> bool {
    match counts.keys().find(|&&x| counts[&x] == n) {
        Some(_) => true,
        None => false,
    }
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
    fn test_counting1() {
        let input = String::from("ababab");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: false,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting2() {
        let input = String::from("abcdee");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: true,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting3() {
        let input = String::from("abcdef");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: false,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting4() {
        let input = String::from("bababc");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: true,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting5() {
        let input = String::from("abbcde");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: true,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting6() {
        let input = String::from("abcccd");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: false,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting7() {
        let input = String::from("aabcdd");
        let id = Id::from(input);
        assert_eq!(
            id,
            Id {
                twice: true,
                thrice: false
            }
        )
    }

    #[test]
    fn test_sample() {
        let input = read_input("sample.txt");
        let checksum = calc_checksum(input);
        assert_eq!(checksum, 12)
    }
}
