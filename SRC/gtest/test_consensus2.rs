#[cfg(test)]
mod tests {
    use crate::consensus::Params;

    #[test]
    fn upgrade_params() {
        let params = Params::default();
        assert!(!params.upgrade_active(Upgrade::Overwinter, 0));
        assert!(params.upgrade_active(Upgrade::Overwinter, 10));
    }
}

pub mod consensus {
    #[derive(PartialEq, Eq, Hash)]
    pub enum Upgrade {
        Overwinter,
        Sapling,
        Blossom,
    }

    pub struct Params {
        pub upgrades: std::collections::HashMap<Upgrade, u32>,
    }

    impl Params {
        pub fn upgrade_active(&self, upgrade: Upgrade, height: u32) -> bool {
            self.upgrades.get(&upgrade).map_or(false, |&h| height >= h)
        }
    }

    impl Default for Params {
        fn default() -> Self {
            let mut upgrades = std::collections::HashMap::new();
            upgrades.insert(Upgrade::Overwinter, 10);
            upgrades.insert(Upgrade::Sapling, 100);
            Params { upgrades }
        }
    }
}
