use std::collections::HashSet;

pub struct Freq {
    shift_list: Vec<i32>,
    repeat: Option<i32>,
}

impl Freq {
    pub fn new(shift_list: Vec<i32>) -> Self {
        Freq {
            shift_list,
            repeat: None,
        }
    }

    pub fn repeating_delta(&mut self) -> i32 {
        self.compute_repeating_delta();
        self.repeat.unwrap()
    }

    fn compute_repeating_delta(&mut self) {
        for _ in self {}
    }
}

pub struct FreqIterator<'a> {
    freq: &'a mut Freq,
    current_index: usize,
    current_delta: i32,
    change_set: HashSet<i32>,
}

impl FreqIterator<'_> {
    fn inc_index(&mut self) {
        let len = self.freq.shift_list.len();
        self.current_index = (self.current_index + 1) % len;
    }
}

impl<'a> Iterator for FreqIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current_item = self.freq.shift_list[self.current_index];
        let new_change = self.current_delta + current_item;
        // println!("{} + {} = {}", self.current_delta, current_item, new_change);
        if self.change_set.insert(new_change) {
            self.current_delta = new_change;
            self.inc_index();
            Some(current_item)
        } else {
            self.freq.repeat = Some(new_change);
            None
        }
    }
}

impl<'a> IntoIterator for &'a mut Freq {
    type Item = i32;
    type IntoIter = FreqIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let current_delta = 0;
        let mut change_set: HashSet<i32> = HashSet::new();
        change_set.insert(current_delta);
        FreqIterator {
            freq: self,
            current_index: 0,
            current_delta,
            change_set,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1, -2, 3, 1];
        let mut freq = Freq::new(v);
        for _ in &mut freq {}
        assert_eq!(freq.repeat, Some(2))
    }

    #[test]
    fn test2() {
        let v = vec![1, -1];
        let mut freq = Freq::new(v);
        for _ in &mut freq {}
        assert_eq!(freq.repeat, Some(0))
    }

    #[test]
    fn test3() {
        let v = vec![3, 3, 4, -2, -4];
        let mut freq = Freq::new(v);
        for _ in &mut freq {}
        assert_eq!(freq.repeat, Some(10))
    }

    #[test]
    fn test4() {
        let v = vec![-6, 3, 8, 5, -6];
        let mut freq = Freq::new(v);
        for _ in &mut freq {}
        assert_eq!(freq.repeat, Some(5))
    }

    #[test]
    fn test5() {
        let mut freq = Freq::new(vec![7, 7, -2, -7, -4]);
        let result = &mut freq.repeating_delta();
        assert_eq!(*result, 14)
    }

    #[test]
    fn test6() {
        let mut freq = Freq::new(vec![-24, 12, 10, 5]);
        let result = &mut freq.repeating_delta();
        assert_eq!(*result, -12)
    }
}
