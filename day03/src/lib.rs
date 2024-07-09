mod cell;
mod claim;
mod fabric;

use claim::Claim;
use std::fs;

fn read_input(path: &str) -> Vec<Claim> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|l| Claim::try_from(l))
        .enumerate()
        .filter_map(|(i, r)| match r {
            Ok(claim) => Some(claim),
            Err(message) => {
                eprintln!("Error in line {}: {}", i + 1, message);
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let expected_input = vec![
            Claim::new("1", (1, 3), 4, 4),
            Claim::new("2", (3, 1), 4, 4),
            Claim::new("3", (5, 5), 2, 2),
        ];
        let input = read_input("sample.txt");
        assert_eq!(input, expected_input);
    }

    #[test]
    fn test_bad_sample() {
        let expected_input = vec![Claim::new("1", (1, 3), 4, 4), Claim::new("3", (5, 5), 2, 2)];
        let input = read_input("bad_sample.txt");
        assert_eq!(input, expected_input);
    }
}
