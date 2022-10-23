use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut dup: HashSet<i32> = HashSet::new();
        let mut res = vec![0, 0];
        let len = nums.len();

        for num in nums {
            if dup.contains(&num){
                res[0] = num;
            }else{
                dup.insert(num);
            }
        }

        for c in 1..=len as i32{
            if !dup.contains(&c){
                res[1] = c as i32;
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1(){
        let nums = vec![1,2,2,4];
        let actual = Solution::find_error_nums(nums);
        let expected = vec![2,3];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2(){
        let nums = vec![1,1];
        let actual = Solution::find_error_nums(nums);
        let expected = vec![1,2];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3(){
        let nums = vec![3,2,3,4,6,5];
        let actual = Solution::find_error_nums(nums);
        let expected = vec![3,1];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test4(){
        let nums = vec![1,5,3,2,2,7,6,4,8,9];
        let actual = Solution::find_error_nums(nums);
        let expected = vec![2,10];
        assert_eq!(expected, actual);
    }
}