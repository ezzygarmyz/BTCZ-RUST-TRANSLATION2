use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{http::Response, Filter};

/// Represents an RPC request.
#[derive(Deserialize)]
struct RpcRequest {
    method: String,
    params: serde_json::Value,
    id: Option<u64>,
}

/// Represents an RPC response.
#[derive(Serialize)]
struct RpcResponse {
    result: Option<serde_json::Value>,
    error: Option<String>,
    id: Option<u64>,
}

/// Starts the RPC server.
pub async fn start_rpc_server(state: Arc<RwLock<ServerState>>) {
    let state_filter = warp::any().map(move || Arc::clone(&state));

    let rpc_route = warp::path("rpc")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter)
        .and_then(handle_rpc_request);

    warp::serve(rpc_route).run(([127, 0, 0, 1], 8232)).await;
}

/// Handles an incoming RPC request.
async fn handle_rpc_request(
    req: RpcRequest,
    state: Arc<RwLock<ServerState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = match req.method.as_str() {
        "getblockchaininfo" => {
            let state = state.read().await;
            Ok(json!({
                "chain": state.chain,
                "blocks": state.blocks,
                "headers": state.headers,
            }))
        }
        _ => Err("Method not found"),
    };

    let response = match result {
        Ok(data) => RpcResponse {
            result: Some(data),
            error: None,
            id: req.id,
        },
        Err(err) => RpcResponse {
            result: None,
            error: Some(err.to_string()),
            id: req.id,
        },
    };

    Ok(warp::reply::json(&response))
}

/// Server state shared across RPC handlers.
#[derive(Default)]
pub struct ServerState {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_rpc_server() {
        let state = Arc::new(RwLock::new(ServerState {
            chain: "main".to_string(),
            blocks: 100,
            headers: 100,
        }));

        let state_filter = warp::any().map(move || Arc::clone(&state));

        let rpc_route = warp::path("rpc")
            .and(warp::post())
            .and(warp::body::json())
            .and(state_filter)
            .and_then(handle_rpc_request);

        let req = json!({
            "method": "getblockchaininfo",
            "params": [],
            "id": 1
        });

        let resp = request()
            .method("POST")
            .path("/rpc")
            .json(&req)
            .reply(&rpc_route)
            .await;

        assert_eq!(resp.status(), 200);
        let body: RpcResponse = serde_json::from_slice(resp.body()).unwrap();
        assert!(body.result.is_some());
        assert_eq!(body.error, None);
        assert_eq!(body.id, Some(1));
    }
}
