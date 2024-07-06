mod frequency;
use std::fs;

pub fn run(path: &str) -> i32 {
    let input = fs::read_to_string(path).unwrap();
    let shift: i32 = input.lines().map(|x| x.parse::<i32>().unwrap()).sum();
    shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_sample() {
        let result = run("sample.txt");
        assert_eq!(result, -7);
    }
}
