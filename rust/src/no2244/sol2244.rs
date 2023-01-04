use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        for task in tasks {
            match count.get(&task) {
                Some(&c) => {
                    count.insert(task, c + 1);
                    ()
                }
                None => {
                    count.insert(task, 1);
                    ()
                }
            }
        }

        let mut round = 0;
        for &c in count.values() {
            if c == 1 {
                return -1;
            }

            if c % 3 == 1 {
                round += c / 3 + 1;
            } else {
                round += (c + 3 - 1) / 3;
            }
        }

        round
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        let actual = Solution::minimum_rounds(tasks);
        let expected = 4;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let tasks = vec![2, 3, 3];

        let actual = Solution::minimum_rounds(tasks);
        let expected = -1;
        assert_eq!(expected, actual);
    }
}
