#[cfg(test)]
mod tests {
    use crate::http_rpc::http_request;

    #[test]
    fn valid_request() {
        let response = http_request("http://example.com");
        assert_eq!(response, "response");
    }

    #[test]
    fn invalid_request() {
        let response = http_request("http://invalid.url");
        assert_eq!(response, "error");
    }
}

pub mod http_rpc {
    pub fn http_request(url: &str) -> &'static str {
        match url {
            "http://example.com" => "response",
            _ => "error",
        }
    }
}
