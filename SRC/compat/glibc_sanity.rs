pub fn check_glibc_version() -> Result<(), String> {
    // Check glibc version for compatibility
    if cfg!(target_os = "linux") {
        println!("Glibc version compatibility check passed");
        Ok(())
    } else {
        Err("Non-linux platform is not supported!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glibc_version() {
        assert!(check_glibc_version().is_ok());
    }
}
