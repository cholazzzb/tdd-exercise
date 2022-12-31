use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

const START_POS: i32 = 0;
const NO_PATH_FOUND: i32 = -1;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, f: i32, b: i32, dst: i32) -> i32 {
        let max_forward = forbidden.iter().copied().max().unwrap_or(0) + f + b + dst;

        let mut visited = HashMap::new();
        for pos in forbidden.into_iter() {
            visited.insert(pos, true);
        }
        visited.insert(START_POS, true);

        let mut queue = vec![];
        let mut neighbours = vec![];

        let mut dist = 0;
        queue.push((START_POS, true));

        while !queue.is_empty() {
            while let Some((pos, can_jump_back)) = queue.pop() {
                if pos == dst {
                    return dist;
                }

                let forward = pos + f;
                if forward < max_forward {
                    let push = match visited.entry(forward) {
                        Entry::Occupied(mut e) => !e.insert(true),
                        Entry::Vacant(e) => *e.insert(true),
                    };

                    if push {
                        neighbours.push((forward, true));
                    }
                }

                let backward = pos - b;
                if can_jump_back && backward > 0 {
                    let push = match visited.entry(backward) {
                        Entry::Occupied(_) => false,
                        Entry::Vacant(e) => !*e.insert(false),
                    };

                    if push {
                        neighbours.push((backward, false));
                    }
                }
            }

            std::mem::swap(&mut queue, &mut neighbours);
            dist += 1;
        }

        NO_PATH_FOUND
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
