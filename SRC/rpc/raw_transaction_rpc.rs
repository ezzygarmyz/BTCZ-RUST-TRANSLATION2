use crate::blockchain::{Blockchain, Transaction};
use crate::rpc::{RpcRequest, RpcResponse, RpcError};
use crate::utils::hex::decode_hex;
use serde_json::json;

/// Handles raw transaction-related RPC requests
pub struct RawTransactionRpc {
    blockchain: Blockchain,
}

impl RawTransactionRpc {
    /// Creates a new RawTransactionRpc handler
    pub fn new(blockchain: Blockchain) -> Self {
        RawTransactionRpc { blockchain }
    }

    /// Handles incoming RPC requests
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            "createrawtransaction" => self.create_raw_transaction(request),
            "decoderawtransaction" => self.decode_raw_transaction(request),
            "sendrawtransaction" => self.send_raw_transaction(request),
            _ => RpcResponse::error(RpcError::method_not_found(request.method)),
        }
    }

    /// Creates a new raw transaction
    fn create_raw_transaction(&self, request: RpcRequest) -> RpcResponse {
        if let Some(inputs) = request.params.get(0).and_then(|p| p.as_array()) {
            if let Some(outputs) = request.params.get(1).and_then(|p| p.as_object()) {
                let mut tx = Transaction::new();

                // Add inputs
                for input in inputs {
                    if let Some(txid) = input.get("txid").and_then(|v| v.as_str()) {
                        if let Some(vout) = input.get("vout").and_then(|v| v.as_u64()) {
                            tx.add_input(txid.to_string(), vout as u32);
                        }
                    }
                }

                // Add outputs
                for (address, amount) in outputs {
                    if let Some(value) = amount.as_f64() {
                        tx.add_output(address.to_string(), value);
                    }
                }

                // Serialize transaction to hex
                match tx.serialize_to_hex() {
                    Ok(hex) => RpcResponse::success(json!({ "hex": hex })),
                    Err(e) => RpcResponse::error(RpcError::internal_error(e.to_string())),
                }
            } else {
                RpcResponse::error(RpcError::invalid_params("Missing outputs"))
            }
        } else {
            RpcResponse::error(RpcError::invalid_params("Missing inputs"))
        }
    }

    /// Decodes a raw transaction into human-readable format
    fn decode_raw_transaction(&self, request: RpcRequest) -> RpcResponse {
        if let Some(hex) = request.params.get(0).and_then(|p| p.as_str()) {
            match decode_hex(hex).and_then(|bytes| Transaction::deserialize(&bytes)) {
                Ok(tx) => RpcResponse::success(json!(tx)),
                Err(_) => RpcResponse::error(RpcError::invalid_params("Invalid transaction hex")),
            }
        } else {
            RpcResponse::error(RpcError::invalid_params("Missing transaction hex"))
        }
    }

    /// Sends a raw transaction to the network
    fn send_raw_transaction(&self, request: RpcRequest) -> RpcResponse {
        if let Some(hex) = request.params.get(0).and_then(|p| p.as_str()) {
            match decode_hex(hex).and_then(|bytes| Transaction::deserialize(&bytes)) {
                Ok(tx) => {
                    if self.blockchain.add_transaction(tx) {
                        RpcResponse::success(json!({ "status": "Transaction added to mempool" }))
                    } else {
                        RpcResponse::error(RpcError::internal_error("Failed to add transaction"))
                    }
                }
                Err(_) => RpcResponse::error(RpcError::invalid_params("Invalid transaction hex")),
            }
        } else {
            RpcResponse::error(RpcError::invalid_params("Missing transaction hex"))
        }
    }
}
