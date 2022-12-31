use std::collections::{HashSet, VecDeque};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let right_bound = x.max(*forbidden.iter().max().unwrap()) + (a + b) * 2;
        let forbidden: HashSet<i32> = forbidden.into_iter().collect();
        let mut visited: HashSet<(i32, bool)> = [(0, false)].into();
        let mut queue = VecDeque::from([(0, 0, false)]);
        while let Some((v, cost, was_back_jump)) = queue.pop_front() {
            if v == x {
                return cost;
            }
            if !forbidden.contains(&(v + a))
                && !visited.contains(&(v + a, false))
                && v + a < right_bound
            {
                visited.insert((v + a, false));
                queue.push_back((v + a, cost + 1, false));
            }
            if !was_back_jump
                && v - b >= 0
                && !forbidden.contains(&(v - b))
                && !visited.contains(&(v - b, true))
            {
                visited.insert((v - b, true));
                queue.push_back((v - b, cost + 1, true));
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let forbidden = vec![14, 4, 18, 1, 15];
        let a = 3;
        let b = 15;
        let x = 9;
        let actual = Solution::minimum_jumps(forbidden, a, b, x);
        let expected = 3;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let forbidden = vec![8, 3, 16, 6, 12, 20];
        let a = 15;
        let b = 13;
        let x = 1;
        let actual = Solution::minimum_jumps(forbidden, a, b, x);
        let expected = -1;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let forbidden = vec![1, 6, 2, 14, 5, 17, 4];
        let a = 16;
        let b = 9;
        let x = 7;
        let actual = Solution::minimum_jumps(forbidden, a, b, x);
        let expected = 2;
        assert_eq!(expected, actual);
    }
}
