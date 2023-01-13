struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let graph = {
            let mut graph: Vec<Vec<usize>> = vec![vec![]; parent.len()];
            for i in 1..parent.len() {
                graph[parent[i] as usize].push(i);
            }
            graph
        };
        let (longest_path, _) = Self::dfs(&graph, &s.chars().collect(), 0);
        longest_path
    }
    fn dfs(graph: &Vec<Vec<usize>>, labels: &Vec<char>, node: usize) -> (i32, i32) {
        let (mut longest_path, mut longest_branch) = (1, 1);
        for child in &graph[node] {
            let (child_path, child_branch) = Self::dfs(graph, labels, *child);
            longest_path = longest_path.max(child_path);
            if labels[*child] == labels[node] {
                continue;
            }
            longest_path = longest_path.max(longest_branch + child_branch);
            longest_branch = longest_branch.max(child_branch + 1);
        }
        (longest_path, longest_branch)
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
