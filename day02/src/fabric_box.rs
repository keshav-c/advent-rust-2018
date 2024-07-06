use std::collections::HashMap;

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
}
