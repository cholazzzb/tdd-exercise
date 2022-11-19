struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_straight_line(mut c: Vec<Vec<i32>>) -> bool {
        c.sort();
        let dy = c[1][1] - c[0][1];
        let dx = c[1][0] - c[0][0];
        let mut dx_zero = false;
        let g = match dx {
            0 => {
                dx_zero = true;
                0
            }
            _ => dy / dx,
        };

        for idx in 2..c.len() {
            let dy = c[idx][1] - c[idx - 1][1];
            let dx = c[idx][0] - c[idx - 1][0];
            let straight = match dx_zero {
                true => dx == 0,
                false => {
                    if dx == 0 {
                        return false;
                    }
                    dy / dx == g
                }
            };
            if !straight {
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
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        let actual = Solution::check_straight_line(coordinates);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let arr = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        let actual = Solution::check_straight_line(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let arr = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 4],
            vec![1, 5],
            vec![1, 6],
            vec![1, 7],
        ];
        let actual = Solution::check_straight_line(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test4() {
        let arr = vec![vec![1, 0], vec![1, 1], vec![1, -1]];
        let actual = Solution::check_straight_line(arr);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
