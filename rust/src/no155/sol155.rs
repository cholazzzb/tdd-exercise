use std::cmp;

struct MinStack {
    pub stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
        } else {
            let (_, prev_min) = self.stack.last().unwrap();
            let min = cmp::min(val, *prev_min);
            self.stack.push((val, min));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        let (res, _) = self.stack.last().unwrap();
        *res
    }

    fn get_min(&self) -> i32 {
        let (_, res) =self.stack.last().unwrap();
        *res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        let ret_1 = min_stack.get_min(); // return -3
        assert_eq!(ret_1, -3);
        min_stack.pop();
        min_stack.top(); // return 0
        let ret_2 = min_stack.get_min(); // return -2
        assert_eq!(ret_2, -2);
    }
}
