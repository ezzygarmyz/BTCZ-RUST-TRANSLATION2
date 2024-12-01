use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::Filter;

/// Represents an HTTP RPC request.
#[derive(Deserialize)]
struct HttpRequest {
    method: String,
    params: serde_json::Value,
    id: Option<u64>,
}

/// Represents an HTTP RPC response.
#[derive(Serialize)]
struct HttpResponse {
    result: Option<serde_json::Value>,
    error: Option<String>,
    id: Option<u64>,
}

/// Starts the HTTP RPC server.
pub async fn start_http_rpc() {
    let rpc_route = warp::path("rpc")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_rpc_request);

    warp::serve(rpc_route).run(([127, 0, 0, 1], 8332)).await;
}

/// Handles an HTTP RPC request.
async fn handle_rpc_request(req: HttpRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let result = match req.method.as_str() {
        "getblockchaininfo" => Ok(json!({
            "chain": "main",
            "blocks": 100,
            "headers": 100,
        })),
        _ => Err("Method not found"),
    };

    let response = match result {
        Ok(data) => HttpResponse {
            result: Some(data),
            error: None,
            id: req.id,
        },
        Err(err) => HttpResponse {
            result: None,
            error: Some(err.to_string()),
            id: req.id,
        },
    };

    Ok(warp::reply::json(&response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_http_rpc() {
        let rpc_route = warp::path("rpc")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_rpc_request);

        let req = serde_json::json!({
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
        let body: HttpResponse = serde_json::from_slice(resp.body()).unwrap();
        assert!(body.result.is_some());
        assert_eq!(body.error, None);
        assert_eq!(body.id, Some(1));
    }
}
