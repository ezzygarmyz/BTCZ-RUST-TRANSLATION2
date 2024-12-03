#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_hex_encoding() {
        let data = b"TestHex";
        let encoded = encode_hex(data);
        assert_eq!(encoded, "54657374486578");
        let decoded = decode_hex(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_encoding() {
        let data = b"TestBase64";
        let encoded = encode_base64(data);
        assert_eq!(encoded, "VGVzdEJhc2U2NA==");
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_invalid_hex() {
        assert!(!is_valid_hex("InvalidHexG"));
        assert!(is_valid_hex("abcdef012345"));
    }

    #[test]
    fn test_invalid_base64() {
        assert!(!is_valid_base64("InvalidBase64*"));
        assert!(is_valid_base64("VGVzdA=="));
    }

    #[test]
    fn test_config_parsing() {
        let config_content = "
            key1=value1
            key2 = value2
            # This is a comment
            key3=value3
        ";
        let path = "test_config.txt";
        std::fs::write(path, config_content).unwrap();
        let config = read_config(path).unwrap();
        assert_eq!(config["key1"], "value1");
        assert_eq!(config["key2"], "value2");
        assert_eq!(config["key3"], "value3");
        std::fs::remove_file(path).unwrap();
    }
}
