struct NumArray {
    pub prefix: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len() + 1];
        for idx in 1..=nums.len() {
            prefix[idx] = prefix[idx - 1] + nums[idx - 1];
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[(right + 1) as usize] - self.prefix[left as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);

        let ret_1 = obj.sum_range(0, 2);
        let ret_2 = obj.sum_range(2, 5);
        let ret_3 = obj.sum_range(0, 5);
        assert_eq!(ret_1, 1);
        assert_eq!(ret_2, -1);
        assert_eq!(ret_3, -3);
    }
}
