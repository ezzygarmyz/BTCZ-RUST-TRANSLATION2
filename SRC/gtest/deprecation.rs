#[cfg(test)]
mod tests {
    use crate::util::system::is_feature_enabled;

    #[test]
    fn feature_enabled() {
        assert!(is_feature_enabled("NewFeature"));
    }

    #[test]
    fn feature_disabled() {
        assert!(!is_feature_enabled("OldFeature"));
    }
}

pub mod util {
    pub mod system {
        pub fn is_feature_enabled(feature: &str) -> bool {
            match feature {
                "NewFeature" => true,
                _ => false,
            }
        }
    }
}
