use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
impl MedianFinder {
    fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        match self.small.len().cmp(&self.large.len()) {
            Ordering::Equal => {
                if self.small.is_empty() {
                    self.small.push(num);
                } else {
                    let l = self.large.peek().unwrap().0;
                    if l < num {
                        self.large.push(Reverse(num));
                    } else {
                        self.small.push(num);
                    }
                }
            }
            Ordering::Less => {
                self.large.push(Reverse(num));
                self.small.push(self.large.pop().unwrap().0);
            }
            Ordering::Greater => {
                self.small.push(num);
                self.large.push(Reverse(self.small.pop().unwrap()));
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() == self.large.len() {
            let s = *self.small.peek().unwrap();
            let l = self.large.peek().unwrap().0;

            return (s + l) as f64 / 2.0;
        }

        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as _
        } else {
            self.large.peek().unwrap().0 as _
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert_eq!(1.5, obj.find_median());
        obj.add_num(3);
        assert_eq!(2.0, obj.find_median());
    }
}
