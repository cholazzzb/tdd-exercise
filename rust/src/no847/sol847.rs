use std::collections::{HashMap, VecDeque};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        if graph.len() == 1 && graph[0].len() == 0 {
            return 0;
        };

        let sz = graph.len();
        let finish_mask = (1 << sz) - 1;
        let mut visited: Vec<HashMap<usize, usize>> = vec![HashMap::new(); sz];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        for node in 0..sz {
            q.push_back((node, 1 << node));
        }

        while let Some(el) = q.pop_front() {
            for &next_node in &graph[el.0] {
                let next_mask = el.1 | (1 << next_node);
                match visited[next_node as usize].get(&next_mask) {
                    None => {
                        q.push_back((next_node as usize, next_mask));
                        match visited[el.0].get(&el.1) {
                            Some(&prev) => {
                                visited[next_node as usize].insert(next_mask, prev + 1);
                                ()
                            }
                            None => {
                                visited[next_node as usize].insert(next_mask, 1);
                                ()
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut res = i32::MAX;
        for node in 0..sz {
            if let Some(&len) = visited[node].get(&finish_mask) {
                res = res.min(len as i32);
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
        let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let actual = Solution::shortest_path_length(graph);
        let expected = 4;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
        let actual = Solution::shortest_path_length(graph);
        let expected = 4;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let graph = vec![vec![]];
        let actual = Solution::shortest_path_length(graph);
        let expected = 0;
        assert_eq!(expected, actual);
    }
}
