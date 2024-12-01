#[cfg(test)]
mod tests {
    use crate::rpc::handle_rpc_request;

    #[test]
    fn handle_valid_rpc() {
        assert_eq!(handle_rpc_request("getblockcount"), "42");
    }

    #[test]
    fn handle_invalid_rpc() {
        assert_eq!(handle_rpc_request("invalid_request"), "error");
    }
}

pub mod rpc {
    pub fn handle_rpc_request(request: &str) -> &'static str {
        match request {
            "getblockcount" => "42",
            _ => "error",
        }
    }
}
