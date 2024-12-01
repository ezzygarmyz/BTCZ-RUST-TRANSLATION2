#[cfg(test)]
mod tests {
    use crate::timedata::get_current_time;

    #[test]
    fn current_time() {
        let now = get_current_time();
        assert!(now > 0);
    }
}

pub mod timedata {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn get_current_time() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}
