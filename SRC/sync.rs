use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, Condvar};
use std::time::Duration;
use thiserror::Error;

/// Custom error for synchronization utilities
#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Timed out while waiting for a lock")]
    Timeout,
}

/// A thread-safe recursive mutex
pub struct RecursiveMutex<T> {
    inner: Mutex<T>,
}

impl<T> RecursiveMutex<T> {
    /// Creates a new RecursiveMutex
    pub fn new(data: T) -> Self {
        RecursiveMutex {
            inner: Mutex::new(data),
        }
    }

    /// Locks the mutex and returns a guard
    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock().unwrap()
    }
}

/// A thread-safe read-write lock
pub struct SyncRwLock<T> {
    inner: RwLock<T>,
}

impl<T> SyncRwLock<T> {
    /// Creates a new SyncRwLock
    pub fn new(data: T) -> Self {
        SyncRwLock {
            inner: RwLock::new(data),
        }
    }

    /// Acquires a read lock
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read().unwrap()
    }

    /// Acquires a write lock
    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write().unwrap()
    }
}

/// A condition variable for thread synchronization
pub struct WaitableLock<T> {
    data: Mutex<T>,
    condvar: Condvar,
}

impl<T> WaitableLock<T> {
    /// Creates a new WaitableLock
    pub fn new(data: T) -> Self {
        WaitableLock {
            data: Mutex::new(data),
            condvar: Condvar::new(),
        }
    }

    /// Waits for a condition to be met
    pub fn wait<F>(&self, condition: F) -> MutexGuard<T>
    where
        F: Fn(&T) -> bool,
    {
        let mut guard = self.data.lock().unwrap();
        while !condition(&*guard) {
            guard = self.condvar.wait(guard).unwrap();
        }
        guard
    }

    /// Waits for a condition with a timeout
    pub fn wait_timeout<F>(&self, condition: F, timeout: Duration) -> Result<MutexGuard<T>, SyncError>
    where
        F: Fn(&T) -> bool,
    {
        let mut guard = self.data.lock().unwrap();
        let result = self
            .condvar
            .wait_timeout_while(guard, timeout, |data| !condition(data))
            .unwrap();

        if result.1.timed_out() {
            Err(SyncError::Timeout)
        } else {
            Ok(result.0)
        }
    }

    /// Notifies one waiting thread
    pub fn notify_one(&self) {
        self.condvar.notify_one();
    }

    /// Notifies all waiting threads
    pub fn notify_all(&self) {
        self.condvar.notify_all();
    }
}
