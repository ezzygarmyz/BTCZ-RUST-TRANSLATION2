use crate::uint256::Uint256;

pub struct ConsensusParams {
    pub pow_limit: Uint256,
    pub pow_target_spacing: i64,
    // Additional fields as needed
}

impl ConsensusParams {
    pub fn new(pow_limit: Uint256, pow_target_spacing: i64) -> Self {
        ConsensusParams {
            pow_limit,
            pow_target_spacing,
        }
    }
}
