use std::collections::BinaryHeap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, mut k: i32) -> i32 {
        let mut heap = BinaryHeap::from(piles);

        while k > 0 {
            if let Some(el) = heap.pop() {
                heap.push((el + 1) / 2);
                k -= 1;
            }
        }

        heap.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    #[test]
    fn test1() {
        let piles = vec![5, 4, 9];
        let k = 2;
        let actual = Solution::min_stone_sum(piles, k);
        let expected = 12;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let piles = vec![4, 3, 6, 7];
        let k = 3;
        let actual = Solution::min_stone_sum(piles, k);
        let expected = 12;
        assert_eq!(expected, actual);
    }
}
