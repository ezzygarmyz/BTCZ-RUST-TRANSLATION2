use crate::blockchain::{Blockchain, Block};
use crate::transaction::{Transaction, Utxo};
use crate::utils::hash::{Hash256, Hash160};
use crate::network::http::{HttpRequest, HttpResponse};
use serde_json::json;

/// Handles REST API requests
pub struct RestApi {
    blockchain: Blockchain,
}

impl RestApi {
    pub fn new(blockchain: Blockchain) -> Self {
        RestApi { blockchain }
    }

    /// Processes an HTTP request and returns the response
    pub fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        match request.path.as_str() {
            "/rest/blockhash" => self.handle_blockhash_request(request),
            "/rest/tx" => self.handle_transaction_request(request),
            "/rest/utxo" => self.handle_utxo_request(request),
            _ => HttpResponse::not_found("Invalid endpoint".to_string()),
        }
    }

    /// Handles requests for blockhash by height
    fn handle_blockhash_request(&self, request: HttpRequest) -> HttpResponse {
        if let Some(height) = request.params.get("height").and_then(|h| h.parse::<u64>().ok()) {
            match self.blockchain.get_block_hash_by_height(height) {
                Some(hash) => HttpResponse::ok(json!({ "blockhash": hash }).to_string()),
                None => HttpResponse::not_found("Block not found".to_string()),
            }
        } else {
            HttpResponse::bad_request("Invalid height parameter".to_string())
        }
    }

    /// Handles requests for raw transaction data
    fn handle_transaction_request(&self, request: HttpRequest) -> HttpResponse {
        if let Some(txid) = request.params.get("txid") {
            match self.blockchain.get_transaction(txid) {
                Some(tx) => HttpResponse::ok(json!({ "transaction": tx }).to_string()),
                None => HttpResponse::not_found("Transaction not found".to_string()),
            }
        } else {
            HttpResponse::bad_request("Invalid txid parameter".to_string())
        }
    }

    /// Handles requests for UTXOs by address
    fn handle_utxo_request(&self, request: HttpRequest) -> HttpResponse {
        if let Some(address) = request.params.get("address") {
            match self.blockchain.get_utxos(address) {
                Some(utxos) => HttpResponse::ok(json!({ "utxos": utxos }).to_string()),
                None => HttpResponse::not_found("No UTXOs found".to_string()),
            }
        } else {
            HttpResponse::bad_request("Invalid address parameter".to_string())
        }
    }
}
