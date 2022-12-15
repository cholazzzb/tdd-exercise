use std::collections::BTreeMap;
#[derive(Default)]
struct MyCalendar {
    m: BTreeMap<i32, i32>,
}

#[allow(dead_code)]
impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((&_s, &e)) = self.m.range(..end).next_back() {
            if start < e {
                return false;
            }
        }
        self.m.insert(start, end);
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut calendar = MyCalendar::new();

        let ret_1 = calendar.book(10, 20);
        assert_eq!(ret_1, true);
        let ret_2 = calendar.book(15, 25);
        assert_eq!(ret_2, false);
        let ret_3 = calendar.book(20, 30);
        assert_eq!(ret_3, true);
    }
}
