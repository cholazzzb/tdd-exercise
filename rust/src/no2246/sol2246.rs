use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = parent.len();
        let mut res = 1;
        let mut cnt = vec![0; sz];
        let mut top1 = vec![1; sz];
        let mut top2 = vec![1; sz];

        for i in 1..sz {
            cnt[parent[i] as usize] += 1;
        }

        let mut q = VecDeque::new();
        for i in 1..sz {
            if cnt[i] == 0 {
                q.push_back(i);
            }
        }

        while !q.is_empty() {
            while let Some(i) = q.pop_front() {
                if i == 0 {
                    return res;
                }
                let p = parent[i] as usize;
                let length = 1 + {
                    if s[i] != s[p] {
                        top1[i]
                    } else {
                        0
                    }
                };
                if top1[p] <= length {
                    top2[p] = top1[p];
                    top1[p] = length;
                } else {
                    top2[p] = top2[p].max(length);
                }
                cnt[p] -= 1;
                if cnt[p] == 0 {
                    q.push_back(p);
                    res = res.max(top1[p] + top2[p] - 1);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let parent = vec![-1, 0, 0, 1, 1, 2];
        let s = String::from("abacbe");
        let actual = Solution::longest_path(parent, s);
        let expected = 3;
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let parent = vec![-1, 0, 0, 0];
        let s = String::from("aabc");
        let actual = Solution::longest_path(parent, s);
        let expected = 3;
        assert_eq!(expected, actual)
    }
}
