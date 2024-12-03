use crate::blockchain::{Blockchain, Block};
use crate::consensus::pow::{check_proof_of_work, calculate_next_work_required};
use crate::rpc::{RpcRequest, RpcResponse, RpcError};
use crate::utils::hash::Hash256;
use serde_json::json;

/// Handles mining-related RPC requests
pub struct MiningRpc {
    blockchain: Blockchain,
}

impl MiningRpc {
    /// Creates a new MiningRpc handler
    pub fn new(blockchain: Blockchain) -> Self {
        MiningRpc { blockchain }
    }

    /// Handles incoming RPC requests
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            "getblocktemplate" => self.get_block_template(request),
            "submitblock" => self.submit_block(request),
            "getmininginfo" => self.get_mining_info(),
            _ => RpcResponse::error(RpcError::method_not_found(request.method)),
        }
    }

    /// Returns a block template for mining
    fn get_block_template(&self, _request: RpcRequest) -> RpcResponse {
        match self.blockchain.create_block_template() {
            Some(template) => RpcResponse::success(json!({
                "version": template.version,
                "previousblockhash": template.previous_block_hash,
                "transactions": template.transactions,
                "coinbaseaux": template.coinbase_aux,
                "coinbasevalue": template.coinbase_value,
                "target": template.target,
                "mintime": template.min_time,
                "mutable": template.mutable,
                "noncerange": template.nonce_range,
                "sigoplimit": template.sigop_limit,
                "sizelimit": template.size_limit,
                "curtime": template.cur_time,
                "bits": template.bits,
                "height": template.height,
            })),
            None => RpcResponse::error(RpcError::internal_error("Failed to create block template")),
        }
    }

    /// Submits a mined block
    fn submit_block(&self, request: RpcRequest) -> RpcResponse {
        if let Some(block_data) = request.params.get(0).and_then(|p| p.as_str()) {
            match Block::deserialize_from_hex(block_data) {
                Ok(block) => {
                    if check_proof_of_work(&block.hash(), block.bits, &self.blockchain.consensus_params()) {
                        if self.blockchain.add_block(block) {
                            RpcResponse::success(json!("Block accepted"))
                        } else {
                            RpcResponse::error(RpcError::internal_error("Block rejected"))
                        }
                    } else {
                        RpcResponse::error(RpcError::invalid_params("Invalid proof of work"))
                    }
                }
                Err(_) => RpcResponse::error(RpcError::invalid_params("Invalid block data")),
            }
        } else {
            RpcResponse::error(RpcError::invalid_params("Block data missing"))
        }
    }

    /// Returns mining-related information
    fn get_mining_info(&self) -> RpcResponse {
        let info = self.blockchain.get_mining_info();
        RpcResponse::success(json!({
            "blocks": info.blocks,
            "currentblocksize": info.current_block_size,
            "currentblocktx": info.current_block_tx,
            "difficulty": info.difficulty,
            "networkhashps": info.network_hash_ps,
            "pooledtx": info.pooled_tx,
            "chain": info.chain,
        }))
    }
}
