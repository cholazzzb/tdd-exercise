use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_hashmap = HashMap::with_capacity(s.len());
        let mut t_hashmap = HashMap::with_capacity(t.len());

        for character in s.chars() {
            *s_hashmap.entry(character).or_insert(0) += 1;
        }
        for character in t.chars() {
            *t_hashmap.entry(character).or_insert(0) += 1;
        }

        s_hashmap == t_hashmap
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        let actual = Solution::is_anagram(s, t);
        let expected = true;
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let s = String::from("rat");
        let t = String::from("car");
        let actual = Solution::is_anagram(s, t);
        let expected = false;
        assert_eq!(expected, actual)
    }
}
