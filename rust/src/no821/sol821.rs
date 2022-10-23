use std::cmp;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut res = vec![0; s.len()];
        let mut pos = s.len() as i32 * -1;

        for (idx, c_in_s) in s.chars().enumerate() {
            if c_in_s == c {
                pos = idx as i32;
            }
            res[idx] = idx as i32 - pos;
        }

        for idx in (0..pos as usize).rev() {
            let c_in_s = s.chars().nth(idx).unwrap();
            if c_in_s == c {
                pos = idx as i32;
            }
            res[idx] = cmp::min(res[idx], pos - idx as i32);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let s = "loveleetcode";
        let c = 'e';
        let actual = Solution::shortest_to_char(s.to_string(), c);
        let expected = vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let s = "aaab";
        let c = 'b';
        let actual = Solution::shortest_to_char(s.to_string(), c);
        let expected = vec![3, 2, 1, 0];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let s = "aaba";
        let c = 'b';
        let actual = Solution::shortest_to_char(s.to_string(), c);
        let expected = vec![2, 1, 0, 1];
        assert_eq!(expected, actual);
    }
}
