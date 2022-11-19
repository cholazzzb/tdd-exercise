use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut count_dif = 0;
        let mut dif1: HashSet<char> = HashSet::new();
        let mut dif2: HashSet<char> = HashSet::new();

        let chars1: Vec<_> = s1.chars().collect();
        let chars2: Vec<_> = s2.chars().collect();

        for idx in 0..s1.len() {
            if chars1[idx] != chars2[idx] {
                count_dif += 1;
                dif1.insert(chars1[idx]);
                dif2.insert(chars2[idx]);
            }
            if count_dif > 2 {
                return false;
            }
        }

        for dif in dif1.iter() {
            if !dif2.contains(dif) {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let s1 = String::from("bank");
        let s2 = String::from("kanb");
        let actual = Solution::are_almost_equal(s1, s2);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let s1 = String::from("attack");
        let s2 = String::from("defend");
        let actual = Solution::are_almost_equal(s1, s2);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let s1 = String::from("kelb");
        let s2 = String::from("kelb");
        let actual = Solution::are_almost_equal(s1, s2);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
