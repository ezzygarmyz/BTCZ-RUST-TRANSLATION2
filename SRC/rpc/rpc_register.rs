use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::rpc::{RpcRequest, RpcResponse, RpcError};
use crate::blockchain_rpc::BlockchainRpc;
use crate::mining_rpc::MiningRpc;
use crate::misc_rpc::MiscRpc;
use crate::net_rpc::NetRpc;
use crate::raw_transaction_rpc::RawTransactionRpc;

/// Centralized registry for all RPC commands
pub struct RpcRegistry {
    handlers: Arc<Mutex<HashMap<String, Box<dyn Fn(RpcRequest) -> RpcResponse + Send + Sync>>>>,
}

impl RpcRegistry {
    /// Creates a new, empty RPC registry
    pub fn new() -> Self {
        RpcRegistry {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers all RPC commands with their respective handlers
    pub fn register_all(
        &self,
        blockchain_rpc: Arc<BlockchainRpc>,
        mining_rpc: Arc<MiningRpc>,
        misc_rpc: Arc<MiscRpc>,
        net_rpc: Arc<NetRpc>,
        raw_transaction_rpc: Arc<RawTransactionRpc>,
    ) {
        self.register("getblockchaininfo", move |req| blockchain_rpc.handle_request(req));
        self.register("getblock", move |req| blockchain_rpc.handle_request(req));
        self.register("getblockhash", move |req| blockchain_rpc.handle_request(req));
        self.register("getrawtransaction", move |req| blockchain_rpc.handle_request(req));

        self.register("getblocktemplate", move |req| mining_rpc.handle_request(req));
        self.register("submitblock", move |req| mining_rpc.handle_request(req));
        self.register("getmininginfo", move |req| mining_rpc.handle_request(req));

        self.register("uptime", move |req| misc_rpc.handle_request(req));
        self.register("logging", move |req| misc_rpc.handle_request(req));
        self.register("stop", move |req| misc_rpc.handle_request(req));

        self.register("getpeerinfo", move |req| net_rpc.handle_request(req));
        self.register("addnode", move |req| net_rpc.handle_request(req));
        self.register("disconnectnode", move |req| net_rpc.handle_request(req));

        self.register("createrawtransaction", move |req| raw_transaction_rpc.handle_request(req));
        self.register("decoderawtransaction", move |req| raw_transaction_rpc.handle_request(req));
        self.register("sendrawtransaction", move |req| raw_transaction_rpc.handle_request(req));
    }

    /// Registers a single RPC command with its handler
    pub fn register<F>(&self, method: &str, handler: F)
    where
        F: Fn(RpcRequest) -> RpcResponse + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(method.to_string(), Box::new(handler));
    }

    /// Dispatches an RPC request to the appropriate handler
    pub fn dispatch(&self, request: RpcRequest) -> RpcResponse {
        let handlers = self.handlers.lock().unwrap();
        if let Some(handler) = handlers.get(&request.method) {
            handler(request)
        } else {
            RpcResponse::error(RpcError::method_not_found(request.method))
        }
    }
}
