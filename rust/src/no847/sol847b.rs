use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        if graph.len() <= 1 {
            return 0;
        }
        let target_mask = (1 << graph.len()) - 1;
        let mut seen = vec![vec![false; u16::MAX as usize]; graph.len()];
        let mut cur_queue = VecDeque::new();

        for i in 0..graph.len() {
            seen[i][1 << i] = true;
            cur_queue.push_back((i, 1 << i));
        }

        let mut steps = 0;
        let mut next_queue = VecDeque::new();
        while !cur_queue.is_empty() {
            while let Some((node, mask)) = cur_queue.pop_front() {
                for &next in graph[node].iter() {
                    let next = next as usize;
                    let next_mask = mask | (1 << next);
                    if next_mask == target_mask {
                        return steps + 1;
                    }
                    if !seen[next][next_mask] {
                        seen[next][next_mask] = true;
                        next_queue.push_back((next, next_mask));
                    }
                }
            }
            steps += 1;
            std::mem::swap(&mut cur_queue, &mut next_queue);
            next_queue.clear();
        }

        -1
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
