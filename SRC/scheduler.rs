use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::blockchain::Blockchain;
use crate::mempool::Mempool;

/// Represents a scheduled task with an associated function and execution time
struct ScheduledTask {
    execute_at: Instant,
    task: Box<dyn FnOnce() + Send + 'static>,
}

impl ScheduledTask {
    fn new<F>(delay: Duration, task: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        ScheduledTask {
            execute_at: Instant::now() + delay,
            task: Box::new(task),
        }
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other.execute_at.cmp(&self.execute_at) // Reverse order for min-heap
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.execute_at == other.execute_at
    }
}

impl Eq for ScheduledTask {}

/// BTCZ Scheduler for managing periodic blockchain tasks
pub struct Scheduler {
    tasks: Arc<Mutex<BinaryHeap<ScheduledTask>>>,
    blockchain: Arc<Blockchain>,
    mempool: Arc<Mempool>,
}

impl Scheduler {
    /// Creates a new Scheduler with dependencies
    pub fn new(blockchain: Arc<Blockchain>, mempool: Arc<Mempool>) -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(BinaryHeap::new())),
            blockchain,
            mempool,
        }
    }

    /// Schedules a task to be executed after a delay
    pub fn schedule<F>(&self, delay: Duration, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let scheduled_task = ScheduledTask::new(delay, task);
        self.tasks.lock().unwrap().push(scheduled_task);
    }

    /// Starts the scheduler loop to execute tasks as they become due
    pub fn start(&self) {
        let tasks = Arc::clone(&self.tasks);
        let blockchain = Arc::clone(&self.blockchain);
        let mempool = Arc::clone(&self.mempool);

        thread::spawn(move || loop {
            let now = Instant::now();
            let mut task_guard = tasks.lock().unwrap();

            while let Some(task) = task_guard.peek() {
                if task.execute_at <= now {
                    let task = task_guard.pop().unwrap();
                    (task.task)();
                } else {
                    break;
                }
            }

            drop(task_guard);
            thread::sleep(Duration::from_millis(50));

            // Schedule periodic BTCZ-specific tasks
            Scheduler::schedule_blockchain_sync(&blockchain);
            Scheduler::schedule_mempool_cleanup(&mempool);
        });
    }

    /// Schedules periodic blockchain synchronization
    fn schedule_blockchain_sync(blockchain: &Arc<Blockchain>) {
        let blockchain_clone = Arc::clone(blockchain);
        let delay = Duration::from_secs(10);

        thread::spawn(move || {
            println!("Synchronizing blockchain...");
            blockchain_clone.synchronize();
        });
    }

    /// Schedules periodic mempool cleanup
    fn schedule_mempool_cleanup(mempool: &Arc<Mempool>) {
        let mempool_clone = Arc::clone(mempool);
        let delay = Duration::from_secs(60);

        thread::spawn(move || {
            println!("Cleaning up mempool...");
            mempool_clone.cleanup();
        });
    }
}
