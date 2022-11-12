#[derive(Default)]
struct MedianFinder {
    vec: Vec<i32>,
}

#[allow(dead_code)]
impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        let idx = self.vec.partition_point(|&x| x < num);
        // The above is equivalent to `let idx = s.binary_search(&num).unwrap_or_else(|x| x);`
        self.vec.insert(idx, num);
    }

    fn find_median(&self) -> f64 {
        let len = self.vec.len();
        if len % 2 == 0 {
            (self.vec[len / 2] as f64 + self.vec[len / 2 - 1] as f64) / 2_f64
        } else {
            self.vec[len / 2] as f64
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
