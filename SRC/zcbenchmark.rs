use std::time::Instant;

/// Represents a single benchmark result
#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: f64, // Duration in seconds
}

/// Represents a benchmark test
pub struct Benchmark {
    name: String,
    function: Box<dyn Fn() -> () + Send + Sync>,
}

impl Benchmark {
    /// Creates a new benchmark
    pub fn new<F>(name: &str, function: F) -> Self
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        Benchmark {
            name: name.to_string(),
            function: Box::new(function),
        }
    }

    /// Executes the benchmark and returns the result
    pub fn run(&self) -> BenchmarkResult {
        let start = Instant::now();
        (self.function)();
        let duration = start.elapsed().as_secs_f64();
        BenchmarkResult {
            name: self.name.clone(),
            duration,
        }
    }
}

/// Benchmark suite to register and execute benchmarks
pub struct BenchmarkSuite {
    benchmarks: Vec<Benchmark>,
}

impl BenchmarkSuite {
    /// Creates a new empty benchmark suite
    pub fn new() -> Self {
        BenchmarkSuite {
            benchmarks: Vec::new(),
        }
    }

    /// Registers a benchmark
    pub fn register<F>(&mut self, name: &str, function: F)
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        self.benchmarks.push(Benchmark::new(name, function));
    }

    /// Runs all benchmarks and returns the results
    pub fn run_all(&self) -> Vec<BenchmarkResult> {
        self.benchmarks.iter().map(|b| b.run()).collect()
    }
}
