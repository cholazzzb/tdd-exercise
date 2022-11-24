struct ParkingSystem {
    pub quota: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            quota: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.quota[(car_type - 1) as usize] > 0 {
            self.quota[(car_type - 1) as usize] -= 1;
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut obj = ParkingSystem::new(1,1,0);
        let ret_1 = obj.add_car(1);
        let ret_2 = obj.add_car(2);
        let ret_3 = obj.add_car(3);
        let ret_4 = obj.add_car(1);
        assert_eq!(ret_1, true);
        assert_eq!(ret_2, true);
        assert_eq!(ret_3, false);
        assert_eq!(ret_4, false);
    }
}
