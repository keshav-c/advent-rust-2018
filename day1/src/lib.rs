mod frequency;
use frequency::Freq;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub struct FreqResult {
    pub sum: i32,
    pub repeat: i32,
}

impl fmt::Display for FreqResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(sum: {}, repeat: {})", self.sum, self.repeat)
    }
}

pub fn run(path: &str) -> FreqResult {
    let frequency_list = get_list(path);
    let sum = frequency_list.iter().sum();
    let repeat = get_repeat(frequency_list);
    FreqResult { sum, repeat }
}

fn get_list(path: &str) -> Vec<i32> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn get_repeat(freqs: Vec<i32>) -> i32 {
    let mut freq = Freq::new(freqs);
    freq.repeating_delta()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_sample() {
        let result = run("sample.txt");
        assert_eq!(
            result,
            FreqResult {
                sum: 3,
                repeat: -12
            }
        );
    }
}
