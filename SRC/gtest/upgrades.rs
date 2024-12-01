#[cfg(test)]
mod tests {
    use crate::upgrades::{Upgrade, Params};

    #[test]
    fn upgrade_active() {
        let params = Params::default();
        assert!(!params.upgrade_active(Upgrade::Sapling, 0));
        assert!(params.upgrade_active(Upgrade::Sapling, 500000));
    }
}

pub mod upgrades {
    #[derive(PartialEq, Eq, Hash, Clone)]
    pub enum Upgrade {
        Overwinter,
        Sapling,
    }

    pub struct Params {
        pub activation_heights: std::collections::HashMap<Upgrade, u32>,
    }

    impl Params {
        pub fn upgrade_active(&self, upgrade: Upgrade, height: u32) -> bool {
            self.activation_heights
                .get(&upgrade)
                .map_or(false, |&activation_height| height >= activation_height)
        }
    }

    impl Default for Params {
        fn default() -> Self {
            let mut activation_heights = std::collections::HashMap::new();
            activation_heights.insert(Upgrade::Sapling, 500000);
            Params { activation_heights }
        }
    }
}
