use crate::blockchain::{Blockchain, Block};
use crate::transaction::{Transaction};
use crate::utils::hash::Hash256;
use crate::rpc::{RpcRequest, RpcResponse, RpcError};

/// Handles blockchain-related RPC requests
pub struct BlockchainRpc {
    blockchain: Blockchain,
}

impl BlockchainRpc {
    /// Creates a new BlockchainRpc handler
    pub fn new(blockchain: Blockchain) -> Self {
        BlockchainRpc { blockchain }
    }

    /// Handles incoming RPC requests
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            "getblockchaininfo" => self.get_blockchain_info(),
            "getblock" => self.get_block(request),
            "getblockhash" => self.get_block_hash(request),
            "getrawtransaction" => self.get_raw_transaction(request),
            _ => RpcResponse::error(RpcError::method_not_found(request.method)),
        }
    }

    /// Returns blockchain information
    fn get_blockchain_info(&self) -> RpcResponse {
        let info = self.blockchain.get_info();
        RpcResponse::success(serde_json::json!({
            "chain": info.chain,
            "blocks": info.blocks,
            "bestblockhash": info.best_block_hash,
            "difficulty": info.difficulty,
        }))
    }

    /// Returns a block by hash or height
    fn get_block(&self, request: RpcRequest) -> RpcResponse {
        if let Some(hash_or_height) = request.params.get(0) {
            match self.blockchain.get_block(hash_or_height) {
                Some(block) => RpcResponse::success(serde_json::json!(block)),
                None => RpcResponse::error(RpcError::block_not_found(hash_or_height.to_string())),
            }
        } else {
            RpcResponse::error(RpcError::invalid_params())
        }
    }

    /// Returns the block hash for a given height
    fn get_block_hash(&self, request: RpcRequest) -> RpcResponse {
        if let Some(height) = request.params.get(0).and_then(|p| p.as_u64()) {
            match self.blockchain.get_block_hash_by_height(height) {
                Some(hash) => RpcResponse::success(serde_json::json!(hash)),
                None => RpcResponse::error(RpcError::block_not_found(height.to_string())),
            }
        } else {
            RpcResponse::error(RpcError::invalid_params())
        }
    }

    /// Returns raw transaction data by hash
    fn get_raw_transaction(&self, request: RpcRequest) -> RpcResponse {
        if let Some(txid) = request.params.get(0).and_then(|p| p.as_str()) {
            match self.blockchain.get_transaction(txid) {
                Some(tx) => RpcResponse::success(serde_json::json!(tx)),
                None => RpcResponse::error(RpcError::tx_not_found(txid.to_string())),
            }
        } else {
            RpcResponse::error(RpcError::invalid_params())
        }
    }
}
