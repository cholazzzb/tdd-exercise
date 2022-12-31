struct Solution;
#[allow(dead_code)]

impl Solution {
    pub fn walk(map: &Vec<Vec<(usize, usize)>>, start: usize, from: usize) -> i32 {
        let mut redirections: i32 = 0;
        for &connection in &map[start] {
            if connection.0 != from && connection.1 != from {
                if connection.0 != start {
                    redirections += Self::walk(map, connection.0, start);
                } else {
                    redirections += 1;
                    redirections += Self::walk(map, connection.1, start);
                }
            }
        }
        redirections
    }
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut connections_map: Vec<Vec<(usize, usize)>> = vec![vec![]; n as usize];
        for connection in connections {
            let tuple: (usize, usize) = (connection[0] as usize, connection[1] as usize);
            connections_map[tuple.0].push(tuple);
            connections_map[tuple.1].push(tuple);
        }
        Self::walk(&connections_map, 0, usize::MAX)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
        let actual = Solution::min_reorder(n, connections);
        let expected = 3;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let n = 5;
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
        let actual = Solution::min_reorder(n, connections);
        let expected = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let n = 3;
        let connections = vec![vec![1, 0], vec![2, 0]];
        let actual = Solution::min_reorder(n, connections);
        let expected = 0;
        assert_eq!(expected, actual);
    }
}
