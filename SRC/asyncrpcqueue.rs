use crate::asyncrpcoperation::{AsyncRPCOperation, AsyncRPCState};
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

/// Represents a queue for managing asynchronous RPC operations.
pub struct AsyncRPCQueue {
    queue: mpsc::Sender<Arc<AsyncRPCOperation>>,
    active_operations: Arc<Mutex<Vec<Arc<AsyncRPCOperation>>>>,
}

impl AsyncRPCQueue {
    /// Creates a new queue with a fixed capacity.
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<Arc<AsyncRPCOperation>>) {
        let (tx, rx) = mpsc::channel(capacity);
        (
            AsyncRPCQueue {
                queue: tx,
                active_operations: Arc::new(Mutex::new(Vec::new())),
            },
            rx,
        )
    }

    /// Adds an operation to the queue.
    pub async fn add_operation(&self, operation: Arc<AsyncRPCOperation>) {
        let mut active_ops = self.active_operations.lock().await;
        active_ops.push(operation.clone());
        self.queue.send(operation).await.unwrap();
    }

    /// Processes all operations in the queue asynchronously.
    pub async fn process_operations(
        mut rx: mpsc::Receiver<Arc<AsyncRPCOperation>>,
    ) {
        while let Some(operation) = rx.recv().await {
            operation.start_operation(async {
                // Simulate operation logic
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                Ok(())
            }).await;
        }
    }

    /// Cancels all operations in the queue.
    pub async fn cancel_all(&self) {
        let mut active_ops = self.active_operations.lock().await;
        for operation in active_ops.iter() {
            operation.cancel_operation();
        }
        active_ops.clear();
    }

    /// Returns the current status of all operations.
    pub async fn get_status(&self) -> Vec<AsyncRPCState> {
        let active_ops = self.active_operations.lock().await;
        active_ops.iter().map(|op| op.get_status()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asyncrpcoperation::AsyncRPCOperation;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_async_rpc_queue() {
        let (queue, rx) = AsyncRPCQueue::new(10);

        // Simulate processing in the background
        tokio::spawn(async move {
            AsyncRPCQueue::process_operations(rx).await;
        });

        // Add operations to the queue
        let op1 = Arc::new(AsyncRPCOperation::new());
        let op2 = Arc::new(AsyncRPCOperation::new());

        queue.add_operation(op1.clone()).await;
        queue.add_operation(op2.clone()).await;

        // Check initial statuses
        let statuses = queue.get_status().await;
        assert!(statuses.iter().all(|s| *s == AsyncRPCState::Pending));

        // Wait for operations to complete
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let statuses = queue.get_status().await;
        assert!(statuses.iter().all(|s| *s == AsyncRPCState::Completed));
    }

    #[tokio::test]
    async fn test_cancel_all() {
        let (queue, rx) = AsyncRPCQueue::new(10);

        // Simulate processing in the background
        tokio::spawn(async move {
            AsyncRPCQueue::process_operations(rx).await;
        });

        // Add operations to the queue
        let op1 = Arc::new(AsyncRPCOperation::new());
        let op2 = Arc::new(AsyncRPCOperation::new());

        queue.add_operation(op1.clone()).await;
        queue.add_operation(op2.clone()).await;

        // Cancel all operations
        queue.cancel_all().await;

        let statuses = queue.get_status().await;
        assert!(statuses.iter().all(|s| *s == AsyncRPCState::Cancelled));
    }
}
