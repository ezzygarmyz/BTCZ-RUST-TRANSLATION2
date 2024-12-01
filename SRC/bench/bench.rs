use std::time::{Duration, Instant};

/// A single benchmark, with a name and a function to execute.
pub struct Benchmark {
    pub name: String,
    pub function: Box<dyn Fn()>,
}

impl Benchmark {
    pub fn new(name: &str, function: Box<dyn Fn()>) -> Self {
        Benchmark {
            name: name.to_string(),
            function,
        }
    }

    pub fn run(&self) -> Duration {
        let start = Instant::now();
        (self.function)();
        start.elapsed()
    }
}

/// The main benchmarking suite.
pub struct BenchSuite {
    pub benchmarks: Vec<Benchmark>,
}

impl BenchSuite {
    pub fn new() -> Self {
        BenchSuite {
            benchmarks: Vec::new(),
        }
    }

    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        self.benchmarks.push(benchmark);
    }

    pub fn run_all(&self) {
        for bench in &self.benchmarks {
            let duration = bench.run();
            println!("Benchmark {}: {:?}", bench.name, duration);
        }
    }
}
