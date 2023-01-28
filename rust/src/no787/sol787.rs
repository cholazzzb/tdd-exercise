struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut costs = vec![u16::MAX; n as usize];
        costs[src as usize] = 0;

        let mut newCosts = Vec::with_capacity(n as usize);
        for _ in 0..=k {
            newCosts.clear();
            newCosts.extend_from_slice(&costs[..]);
            for flight in flights.iter() {
                let (from, to, price) = (flight[0] as usize, flight[1] as usize, flight[2] as u16);
                if costs[from] != u16::MAX {
                    newCosts[to] = newCosts[to].min(costs[from] + price);
                }
            }
            std::mem::swap(&mut costs, &mut newCosts);
        }

        match costs[dst as usize] {
            u16::MAX => -1,
            res => res as i32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        let actual = Solution::find_cheapest_price(n, flights, src, dst, k);
        let expected = 700;
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        let actual = Solution::find_cheapest_price(n, flights, src, dst, k);
        let expected = 200;
        assert_eq!(expected, actual)
    }
    #[test]
    fn test3() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        let actual = Solution::find_cheapest_price(n, flights, src, dst, k);
        let expected = 500;
        assert_eq!(expected, actual)
    }
}
