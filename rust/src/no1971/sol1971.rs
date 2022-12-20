use std::{collections::HashSet, vec};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let graphs: Vec<HashSet<usize>> = {
            let mut temp = vec![HashSet::new(); n as usize];

            for edge in edges.iter() {
                let l = edge[0] as usize;
                let r = edge[1] as usize;
                temp[l].insert(r);
                temp[r].insert(l);
            }

            temp
        };
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(source as usize);
        Self::dfs(
            source as usize,
            destination as usize,
            &mut seen,
            & graphs,
        )
    }

    fn dfs(
        cur: usize,
        destination: usize,
        seen: &mut HashSet<usize>,
        graph: &Vec<HashSet<usize>>,
    ) -> bool {
        if cur == destination {
            return true;
        }

        for &next in &graph[cur] {
            if seen.insert(next) {
                if Self::dfs(next, destination, seen, graph) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let actual = Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2);
        let expected = true;
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let actual = Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5,
        );
        let expected = false;
        assert_eq!(expected, actual)
    }
}
