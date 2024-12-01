#[cfg(test)]
mod tests {
    use crate::random::generate_random_number;

    #[test]
    fn generate_random_number() {
        let rand_num = generate_random_number(1, 10);
        assert!(rand_num >= 1 && rand_num <= 10);
    }
}

pub mod random {
    use rand::Rng;

    pub fn generate_random_number(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }
}
