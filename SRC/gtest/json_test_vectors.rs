#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    #[test]
    fn basic_read_write() {
        // Create a JSON object
        let obj = json!({
            "key": "value"
        });

        // Convert the JSON object to a string
        let json_str = serde_json::to_string_pretty(&obj).unwrap();
        assert_eq!(json_str, "{\n  \"key\": \"value\"\n}");

        // Parse the JSON string back into a JSON value
        let parsed_value: Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed_value.is_object());
    }
}
