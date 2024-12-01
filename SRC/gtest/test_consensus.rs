#[cfg(test)]
mod tests {
    use crate::consensus::Params;

    #[test]
    fn default_params() {
        let params = Params::default();
        assert_eq!(params.pow_target_timespan, 14 * 24 * 60 * 60);
    }
}

pub mod consensus {
    pub struct Params {
        pub pow_target_timespan: u64,
    }

    impl Default for Params {
        fn default() -> Self {
            Params {
                pow_target_timespan: 14 * 24 * 60 * 60,
            }
        }
    }
}
