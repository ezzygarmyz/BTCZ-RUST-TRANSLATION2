use std::sync::{Mutex, MutexGuard};

/// A utility for temporarily releasing and reacquiring a lock
pub struct ReverseLock<'a, T> {
    original_lock: &'a Mutex<T>,
    temporary_lock: Option<MutexGuard<'a, T>>,
}

impl<'a, T> ReverseLock<'a, T> {
    /// Temporarily releases the lock and reacquires it later
    pub fn new(lock: &'a Mutex<T>, guard: MutexGuard<'a, T>) -> Self {
        drop(guard); // Explicitly release the lock
        ReverseLock {
            original_lock: lock,
            temporary_lock: None,
        }
    }

    /// Reacquires the lock
    pub fn reacquire(&mut self) {
        if self.temporary_lock.is_none() {
            self.temporary_lock = Some(self.original_lock.lock().unwrap());
        }
    }
}

impl<'a, T> Drop for ReverseLock<'a, T> {
    /// Ensures the lock is reacquired when the `ReverseLock` goes out of scope
    fn drop(&mut self) {
        self.reacquire();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_reverse_lock() {
        let data = Arc::new(Mutex::new(42));

        let lock = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let guard = lock.lock().unwrap();
            println!("Lock acquired in thread, value: {}", *guard);

            let mut reverse_lock = ReverseLock::new(&lock, guard);
            println!("Lock released temporarily in thread");

            // Perform some operations while the lock is released
            thread::sleep(std::time::Duration::from_millis(100));

            reverse_lock.reacquire();
            println!("Lock reacquired in thread");
        });

        handle.join().unwrap();
    }
}
