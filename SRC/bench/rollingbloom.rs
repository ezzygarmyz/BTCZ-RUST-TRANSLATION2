use bloomfilter::Bloom;

pub struct RollingBloom {
    bloom: Bloom,
    capacity: usize,
}

impl RollingBloom {
    pub fn new(capacity: usize) -> Self {
        RollingBloom {
            bloom: Bloom::new_for_fp_rate(capacity, 0.01),
            capacity,
        }
    }

    pub fn insert(&mut self, item: &str) {
        self.bloom.set(item);
    }

    pub fn contains(&self, item: &str) -> bool {
        self.bloom.check(item)
    }

    pub fn benchmark(&mut self, items: &[&str]) {
        for item in items {
            self.insert(item);
        }
        println!("RollingBloom Benchmark Complete");
    }
}
