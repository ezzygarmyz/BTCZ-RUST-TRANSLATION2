#[cfg(target_os = "windows")]
mod compat {
    pub fn platform_info() {
        println!("Running on Windows");
    }

    pub fn sleep_ms(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

#[cfg(not(target_os = "windows"))]
mod compat {
    pub fn platform_info() {
        println!("Running on a Unix-like system");
    }

    pub fn sleep_ms(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

#[cfg(target_endian = "little")]
pub fn is_little_endian() -> bool {
    true
}

#[cfg(target_endian = "big")]
pub fn is_little_endian() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_info() {
        compat::platform_info();
    }

    #[test]
    fn test_endianness() {
        if is_little_endian() {
            println!("System is little-endian");
        } else {
            println!("System is big-endian");
        }
    }
}
