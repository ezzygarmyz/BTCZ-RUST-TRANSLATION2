use crate::network::{NetworkManager, PeerInfo};
use crate::rpc::{RpcRequest, RpcResponse, RpcError};
use serde_json::json;

/// Handles networking-related RPC requests
pub struct NetRpc {
    network_manager: NetworkManager,
}

impl NetRpc {
    /// Creates a new NetRpc handler
    pub fn new(network_manager: NetworkManager) -> Self {
        NetRpc { network_manager }
    }

    /// Handles incoming RPC requests
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            "getpeerinfo" => self.get_peer_info(),
            "addnode" => self.add_node(request),
            "disconnectnode" => self.disconnect_node(request),
            _ => RpcResponse::error(RpcError::method_not_found(request.method)),
        }
    }

    /// Returns information about connected peers
    fn get_peer_info(&self) -> RpcResponse {
        let peers: Vec<PeerInfo> = self.network_manager.get_peer_info();
        RpcResponse::success(json!(peers))
    }

    /// Adds a node to the network
    fn add_node(&self, request: RpcRequest) -> RpcResponse {
        if let Some(node) = request.params.get(0).and_then(|p| p.as_str()) {
            if self.network_manager.add_node(node) {
                RpcResponse::success(json!({ "status": "node added" }))
            } else {
                RpcResponse::error(RpcError::internal_error("Failed to add node"))
            }
        } else {
            RpcResponse::error(RpcError::invalid_params("Node address missing"))
        }
    }

    /// Disconnects a node from the network
    fn disconnect_node(&self, request: RpcRequest) -> RpcResponse {
        if let Some(node) = request.params.get(0).and_then(|p| p.as_str()) {
            if self.network_manager.disconnect_node(node) {
                RpcResponse::success(json!({ "status": "node disconnected" }))
            } else {
                RpcResponse::error(RpcError::internal_error("Failed to disconnect node"))
            }
        } else {
            RpcResponse::error(RpcError::invalid_params("Node address missing"))
        }
    }
}
