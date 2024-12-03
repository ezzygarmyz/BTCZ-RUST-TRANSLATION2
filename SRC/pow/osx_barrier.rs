use std::sync::{Arc, Barrier};
use std::thread;

pub struct OsxBarrier {
    inner: Arc<Barrier>,
}

impl OsxBarrier {
    /// Creates a new barrier for a specified number of threads
    pub fn new(thread_count: usize) -> Self {
        OsxBarrier {
            inner: Arc::new(Barrier::new(thread_count)),
        }
    }

    /// Waits for all threads to reach the barrier
    pub fn wait(&self) {
        self.inner.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osx_barrier() {
        let barrier = OsxBarrier::new(3);
        let mut handles = vec![];

        for i in 0..3 {
            let barrier_clone = barrier.inner.clone();
            handles.push(thread::spawn(move || {
                println!("Thread {} waiting", i);
                barrier_clone.wait();
                println!("Thread {} proceeding", i);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
