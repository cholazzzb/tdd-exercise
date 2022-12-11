use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct SeatManager {
    seat: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        let mut seat = vec![];
        for i in 1..=n {
            seat.push(Reverse(i));
        }
        return Self {
            seat: BinaryHeap::from(seat),
        };
    }

    fn reserve(&mut self) -> i32 {
        let res = self.seat.pop();
        match res {
            Some(Reverse(val)) => val,
            _ => 0,
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.seat.push(Reverse(seat_number));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut obj = SeatManager::new(5);

        let ret_1 = obj.reserve();
        assert_eq!(ret_1, 1);
        let ret_2 = obj.reserve();
        assert_eq!(ret_2, 2);
        obj.unreserve(2);
        let ret_3 = obj.reserve();
        assert_eq!(ret_3, 2);
        let ret_4 = obj.reserve();
        assert_eq!(ret_4, 3);
        let ret_5 = obj.reserve();
        assert_eq!(ret_5, 4);
        let ret_6 = obj.reserve();
        assert_eq!(ret_6, 5);
        obj.unreserve(5);
    }
}
