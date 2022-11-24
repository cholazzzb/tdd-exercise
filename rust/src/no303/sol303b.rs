struct NumArray {
    prefix: Vec<i32>,
}

#[allow(dead_code)]
impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        nums.iter_mut().fold(0, |acc, num| {
            *num += acc;
            *num
        });

        Self { prefix: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        if left == 0 {
            self.prefix[right]
        } else {
            self.prefix[right] - self.prefix[left - 1]
        }
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
