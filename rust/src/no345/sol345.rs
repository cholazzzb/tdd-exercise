use std::usize;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = vec!['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];

        let mut s: Vec<char> = s.chars().collect();
        let (mut l, mut r): (usize, usize) = (0, s.len() - 1);

        loop {
            while l < r && !vowels.contains(&s[l]) {
                l += 1;
            }
            while l < r && !vowels.contains(&s[r]) {
                r -= 1;
            }

            if l >= r {
                break;
            }

            s.swap(l, r);

            l += 1;
            r -= 1;
        }

        s.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let s = String::from("hello");
        let actual = Solution::reverse_vowels(s);
        let expected = String::from("holle");
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let s = String::from("leetcode");
        let actual = Solution::reverse_vowels(s);
        let expected = String::from("leotcede");
        assert_eq!(expected, actual)
    }
}
