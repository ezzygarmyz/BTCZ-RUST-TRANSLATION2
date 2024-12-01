use crate::bench::{Benchmark, BenchSuite};

pub fn run_example_benchmarks() {
    let mut suite = BenchSuite::new();

    suite.add_benchmark(Benchmark::new("Example 1", Box::new(|| {
        println!("Running example 1...");
    })));

    suite.add_benchmark(Benchmark::new("Example 2", Box::new(|| {
        println!("Running example 2...");
    })));

    suite.run_all();
}
