use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();

        fn check(room_idx: usize, rooms: &Vec<Vec<i32>>, seen: &mut HashSet<usize>) {
            for key in rooms[room_idx].iter() {
                if *key != 0 {
                    if seen.insert(*key as usize) {
                        check(*key as usize, rooms, seen);
                    }
                }
            }
            ()
        }

        check(0, &rooms, &mut seen);

        if seen.len() == rooms.len() - 1 {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    #[test]
    fn test1() {
        let arr = vec![vec![1], vec![2], vec![3], vec![]];
        let actual = Solution::can_visit_all_rooms(arr);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let arr = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        let actual = Solution::can_visit_all_rooms(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
