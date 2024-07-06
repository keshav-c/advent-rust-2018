use std::collections::HashMap;
use std::iter;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub struct FabricBox {
    id: String,
    pub twice: bool,
    pub thrice: bool,
}

impl From<String> for FabricBox {
    fn from(value: String) -> Self {
        FabricBox::new(value)
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
        let input = String::from("ababab");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
                twice: false,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting2() {
        let input = String::from("abcdee");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
                twice: true,
                thrice: false
            }
        );
    }

    #[test]
    fn test_counting3() {
        let input = String::from("abcdef");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
                twice: false,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting4() {
        let input = String::from("bababc");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
                twice: true,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting5() {
        let input = String::from("abbcde");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
                twice: true,
                thrice: false
            }
        )
    }

    #[test]
    fn test_counting6() {
        let input = String::from("abcccd");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
                twice: false,
                thrice: true
            }
        )
    }

    #[test]
    fn test_counting7() {
        let input = String::from("aabcdd");
        let b = FabricBox::from(input.clone());
        assert_eq!(
            b,
            FabricBox {
                id: input,
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
