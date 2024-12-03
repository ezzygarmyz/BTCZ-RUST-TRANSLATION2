use crate::pow::tromp::equi::EquihashContext;

pub struct EquihashMiner {
    context: EquihashContext,
}

impl EquihashMiner {
    /// Initializes a new miner with Equihash parameters
    pub fn new(n: u32, k: u32, seed: [u8; 32]) -> Self {
        EquihashMiner {
            context: EquihashContext::new(n, k, seed),
        }
    }

    /// Mines solutions for the given header
    pub fn mine(&mut self, header: &[u8]) -> Vec<Vec<u8>> {
        self.context.solve(header);
        self.context.solutions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equihash_miner() {
        let header = [0u8; 140]; // Mock header
        let seed = [0u8; 32];

        let mut miner = EquihashMiner::new(200, 9, seed);
        let solutions = miner.mine(&header);

        println!("Found {} solutions", solutions.len());
    }
}
