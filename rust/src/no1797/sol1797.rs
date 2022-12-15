use std::collections::HashMap;

struct AuthenticationManager {
    pub ttl: i32,
    pub map: HashMap<String, i32>,
}

#[allow(dead_code)]
impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        AuthenticationManager {
            ttl: time_to_live,
            map: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.map.insert(token_id, self.ttl + current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(expired_time) = self.map.get_mut(&token_id) {
            if current_time < *expired_time {
                *expired_time = self.ttl + current_time;
            } else {
                self.map.remove(&token_id);
            }
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        let mut expired = vec![];
        for (k, t) in self.map.iter_mut() {
            if current_time >= *t {
                expired.push(k.clone())
            }
        }

        for k in expired {
            self.map.remove(&k);
        }

        self.map.keys().len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut obj = AuthenticationManager::new(5);
        obj.renew(String::from("aaa"), 1);
        obj.generate(String::from("aaa"), 2);
        let ret_1: i32 = obj.count_unexpired_tokens(6);
        assert_eq!(1, ret_1);

        obj.generate(String::from("bbb"), 7);
        obj.renew(String::from("aaa"), 8);
        obj.renew(String::from("bbb"), 10);

        let ret_2: i32 = obj.count_unexpired_tokens(15);
        assert_eq!(0, ret_2);
    }
}
