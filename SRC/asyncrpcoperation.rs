use serde_json::Value as JsonValue;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use std::time::Duration;

/// Represents the status of an asynchronous RPC operation.
#[derive(Debug, Clone, PartialEq)]
pub enum AsyncRPCState {
    Pending,
    InProgress,
    Completed,
    Cancelled,
    Failed(String), // Includes error message
}

/// Represents an asynchronous RPC operation.
pub struct AsyncRPCOperation {
    state: Arc<Mutex<AsyncRPCState>>,
    notify: Arc<Notify>,
}

impl AsyncRPCOperation {
    /// Creates a new asynchronous RPC operation.
    pub fn new() -> Self {
        AsyncRPCOperation {
            state: Arc::new(Mutex::new(AsyncRPCState::Pending)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Starts the asynchronous operation.
    pub async fn start_operation<F>(&self, task: F)
    where
        F: std::future::Future<Output = Result<(), String>> + Send + 'static,
    {
        {
            let mut state = self.state.lock().unwrap();
            *state = AsyncRPCState::InProgress;
        }

        let state = self.state.clone();
        let notify = self.notify.clone();

        tokio::spawn(async move {
            match task.await {
                Ok(_) => {
                    let mut state = state.lock().unwrap();
                    *state = AsyncRPCState::Completed;
                }
                Err(err) => {
                    let mut state = state.lock().unwrap();
                    *state = AsyncRPCState::Failed(err);
                }
            }
            notify.notify_one();
        });
    }

    /// Cancels the asynchronous operation.
    pub fn cancel_operation(&self) {
        let mut state = self.state.lock().unwrap();
        *state = AsyncRPCState::Cancelled;
        self.notify.notify_one();
    }

    /// Waits for the operation to complete or be cancelled.
    pub async fn wait(&self) {
        self.notify.notified().await;
    }

    /// Gets the current status of the operation.
    pub fn get_status(&self) -> AsyncRPCState {
        let state = self.state.lock().unwrap();
        state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time;

    #[tokio::test]
    async fn test_async_rpc_operation() {
        let rpc_op = AsyncRPCOperation::new();

        // Simulate a long-running operation
        let task = async {
            time::sleep(Duration::from_secs(2)).await;
            Ok(())
        };

        rpc_op.start_operation(task).await;

        // Check that the operation is in progress
        assert_eq!(rpc_op.get_status(), AsyncRPCState::InProgress);

        // Wait for the operation to complete
        rpc_op.wait().await;

        // Check that the operation is completed
        assert_eq!(rpc_op.get_status(), AsyncRPCState::Completed);
    }

    #[tokio::test]
    async fn test_cancel_operation() {
        let rpc_op = AsyncRPCOperation::new();

        // Simulate a long-running operation
        let task = async {
            time::sleep(Duration::from_secs(5)).await;
            Ok(())
        };

        rpc_op.start_operation(task).await;

        // Cancel the operation
        rpc_op.cancel_operation();

        // Wait for cancellation to propagate
        rpc_op.wait().await;

        // Check that the operation is cancelled
        assert_eq!(rpc_op.get_status(), AsyncRPCState::Cancelled);
    }
}
