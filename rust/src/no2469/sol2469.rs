struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let mut res = vec![];

        res.push(celsius + 273.15);
        res.push(celsius * 1.80 + 32.00);

        res
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let actual = Solution::convert_temperature(36.50);
        let expected = vec![309.65000,97.70000];
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let actual = Solution::convert_temperature(122.11);
        let expected = vec![395.26000,251.79800];
        assert_eq!(expected, actual)
    }
}
