/// Performs a basic sanity check to ensure the platform is supported.
pub fn sanity_check() -> Result<(), String> {
    if cfg!(target_pointer_width = "64") {
        println!("Sanity check passed: 64-bit platform.");
        Ok(())
    } else {
        Err("Unsupported platform: Only 64-bit architectures are supported.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity_check() {
        assert!(sanity_check().is_ok());
    }
}
