#[cfg(test)]
mod tests {
    use crate::founders_reward::founders_reward;

    #[test]
    fn valid_reward() {
        assert_eq!(founders_reward(1000), 50); // 5% of 1000
    }

    #[test]
    fn zero_reward() {
        assert_eq!(founders_reward(0), 0);
    }
}

pub mod founders_reward {
    pub fn founders_reward(block_reward: u64) -> u64 {
        // Example calculation: 5% founders reward
        (block_reward * 5) / 100
    }
}
