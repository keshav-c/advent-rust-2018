use std::collections::HashMap;
use std::iter;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub struct FabricBox {
    id: String,
    pub twice: bool,
    pub thrice: bool,
}

impl From<&str> for FabricBox {
    fn from(value: &str) -> Self {
        FabricBox::new(value.to_owned())
    }
}

impl FabricBox {
    fn new(id: String) -> Self {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in id.chars() {
            counts.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        Self {
            id,
            twice: has_count(&counts, 2),
            thrice: has_count(&counts, 3),
        }
    }

    pub fn has_similar_id(&self, other: &FabricBox) -> Result<String, ()> {
        let mut unmatched_chars = 0;
        let mut matched_chars = String::new();
        for (x, y) in iter::zip(self, other) {
            if x != y {
                unmatched_chars += 1
            } else {
                matched_chars.push(x)
            }
        }
        if unmatched_chars == 1 {
            Ok(matched_chars)
        } else {
            Err(())
        }
    }
}

impl<'a> IntoIterator for &'a FabricBox {
    type Item = char;
    type IntoIter = Chars<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.id.chars()
    }
}

fn has_count(counts: &HashMap<char, i32>, n: i32) -> bool {
    match counts.keys().find(|&&x| counts[&x] == n) {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting1() {
        let b = FabricBox::from("ababab");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("ababab"),
                twice: false,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting2() {
        let b = FabricBox::from("abcdee");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("abcdee"),
                twice: true,
                thrice: false
            }
        );
    }

    #[test]
    fn test_counting3() {
        let b = FabricBox::from("abcdef");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("abcdef"),
                twice: false,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting4() {
        let b = FabricBox::from("bababc");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("bababc"),
                twice: true,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting5() {
        let b = FabricBox::from("abbcde");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("abbcde"),
                twice: true,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting6() {
        let b = FabricBox::from("abcccd");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("abcccd"),
                twice: false,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting7() {
        let b = FabricBox::from("aabcdd");
        assert_eq!(
            b,
            FabricBox {
                id: String::from("aabcdd"),
                twice: true,
                thrice: false
            }
        )
    }

    #[test]
    fn test_similar_id() {
        let b1 = FabricBox {
            id: String::from("fghij"),
            twice: true,
            thrice: true,
        };
        let b2 = FabricBox {
            id: String::from("fguij"),
            twice: true,
            thrice: true,
        };
        assert_eq!(b1.has_similar_id(&b2), Ok("fgij".into()));
        let b3 = FabricBox {
            id: String::from("fguie"),
            twice: true,
            thrice: true,
        };
        assert_eq!(b1.has_similar_id(&b3), Err(()));
    }
}
