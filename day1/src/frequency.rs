use std::collections::HashSet;

pub struct Freq {
    shift_list: Vec<i32>,
    change_set: HashSet<i32>,
    current_delta: i32,
    repeat: Option<i32>,
    current_index: usize,
}

impl Freq {
    pub fn new(shift_list: Vec<i32>) -> Self {
        let mut change_set: HashSet<i32> = HashSet::new();
        let current_delta = 0;
        change_set.insert(current_delta);
        Freq {
            shift_list,
            change_set,
            current_delta,
            repeat: None,
            current_index: 0usize,
        }
    }

    fn inc_index(&mut self) {
        let len = self.shift_list.len();
        self.current_index = (self.current_index + 1) % len;
    }
}

impl Iterator for Freq {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current_item = self.shift_list[self.current_index];
        let new_change = self.current_delta + current_item;
        if self.change_set.insert(new_change) {
            self.inc_index();
            Some(current_item)
        } else {
            self.repeat = Some(new_change);
            None
        }
    }
}

// impl IntoIterator for &mut Freq {
//     type Item = &i32;
//     type IntoIter = ;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1, -2, 3, 1];
        let mut freq = Freq::new(v);
        for _ in freq {}
        assert_eq!(freq.repeat, Some(2))
    }
}
