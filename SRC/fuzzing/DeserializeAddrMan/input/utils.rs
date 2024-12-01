pub fn validate_ip(ip: &str) -> bool {
    ip.split('.')
        .all(|segment| segment.parse::<u8>().is_ok() && segment.parse::<u8>().unwrap() <= 255)
}
