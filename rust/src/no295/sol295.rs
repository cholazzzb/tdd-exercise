use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        self.hi.push(Reverse(*self.lo.peek().unwrap()));
        self.lo.pop();
        if self.lo.len() < self.hi.len() {
            self.lo.push(self.hi.peek().unwrap().0);
            self.hi.pop();
        }
    }

    fn find_median(&self) -> f64 {
        if self.lo.len() > self.hi.len() {
            return *self.lo.peek().unwrap() as f64;
        }
        (self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.0
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
