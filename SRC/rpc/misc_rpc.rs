use crate::rpc::{RpcRequest, RpcResponse, RpcError};
use crate::logging::{get_logging_info, set_logging_level};
use serde_json::json;
use std::time::Instant;

/// Handles miscellaneous RPC requests
pub struct MiscRpc {
    start_time: Instant,
}

impl MiscRpc {
    /// Creates a new MiscRpc handler
    pub fn new() -> Self {
        MiscRpc {
            start_time: Instant::now(),
        }
    }

    /// Handles incoming RPC requests
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            "uptime" => self.get_uptime(),
            "logging" => self.handle_logging(request),
            "stop" => self.stop_node(),
            _ => RpcResponse::error(RpcError::method_not_found(request.method)),
        }
    }

    /// Returns the uptime of the node
    fn get_uptime(&self) -> RpcResponse {
        let uptime = self.start_time.elapsed().as_secs();
        RpcResponse::success(json!({ "uptime": uptime }))
    }

    /// Handles logging-related commands
    fn handle_logging(&self, request: RpcRequest) -> RpcResponse {
        if let Some(params) = request.params.get(0) {
            if params["action"] == "get" {
                let info = get_logging_info();
                return RpcResponse::success(json!(info));
            } else if params["action"] == "set" {
                if let Some(level) = params["level"].as_str() {
                    set_logging_level(level.to_string());
                    return RpcResponse::success(json!({ "status": "success" }));
                } else {
                    return RpcResponse::error(RpcError::invalid_params("Missing logging level"));
                }
            }
        }
        RpcResponse::error(RpcError::invalid_params("Invalid logging parameters"))
    }

    /// Stops the node
    fn stop_node(&self) -> RpcResponse {
        // For production code, integrate with the system to gracefully shut down the node
        RpcResponse::success(json!({ "status": "Node is shutting down" }))
    }
}
