pub mod params {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DeploymentPos {
        TestDummy,
        CSV,
        SegWit,
    }

    pub const WITNESS_SCALE_FACTOR: usize = 4;
    pub const MIN_TRANSACTION_WEIGHT: usize = WITNESS_SCALE_FACTOR * 60;
    pub const MIN_SERIALIZABLE_TRANSACTION_WEIGHT: usize = WITNESS_SCALE_FACTOR * 10;
}
