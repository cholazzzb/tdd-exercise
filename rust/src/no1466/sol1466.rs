struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut map: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for edge in connections {
            let (v1, v2) = (edge[0], edge[1]);
            map[v1 as usize].push(v2);
            map[v2 as usize].push(v1 * -1);
        }

        let mut visited = vec![false; n as usize];

        Self::dfs(0, &map, &mut visited)
    }

    pub fn dfs(node: usize, map: &Vec<Vec<i32>>, visited: &mut Vec<bool>) -> i32 {
        let mut change = 0;

        visited[node] = true;
        for &next in &map[node] {

            if !visited[next.abs() as usize] {
                change += Self::dfs(next.abs() as usize, map, visited) + {
                    if next > 0 {
                        1
                    } else {
                        0
                    }
                };
            }
        }

        change
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
