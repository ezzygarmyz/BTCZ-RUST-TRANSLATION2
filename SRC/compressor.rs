/// Compresses a transaction output amount.
pub fn compress_amount(amount: u64) -> u64 {
    if amount == 0 {
        0
    } else if amount <= 10 {
        amount + 1
    } else if amount < 1_000_000 {
        let e = (amount as f64).log10().floor() as u64;
        let base = 10u64.pow(e as u32);
        let mantissa = amount / base - 1;
        let exponent = e + 9;
        mantissa * 10 + exponent
    } else {
        amount / 100_000_000 + 100
    }
}

/// Decompresses a compressed transaction output amount.
pub fn decompress_amount(x: u64) -> u64 {
    if x == 0 {
        0
    } else if x <= 10 {
        x - 1
    } else if x < 111 {
        let e = x % 10;
        let mantissa = x / 10 - 1;
        let base = 10u64.pow((e - 9) as u32);
        (mantissa + 1) * base
    } else {
        (x - 100) * 100_000_000
    }
}

/// Compresses a scriptPubKey into a compressed format.
pub fn compress_script(script: &[u8]) -> Option<Vec<u8>> {
    if script.len() == 25 && script[0] == 0x76 && script[1] == 0xa9 && script[2] == 0x14 && script[23] == 0x88 && script[24] == 0xac {
        // P2PKH
        Some(script[3..23].to_vec())
    } else if script.len() == 23 && script[0] == 0xa9 && script[1] == 0x14 && script[22] == 0x87 {
        // P2SH
        Some(script[2..22].to_vec())
    } else {
        None
    }
}

/// Decompresses a scriptPubKey from its compressed format.
pub fn decompress_script(data: &[u8], script_type: u8) -> Option<Vec<u8>> {
    match script_type {
        0 => {
            // P2PKH
            let mut script = vec![0x76, 0xa9, 0x14];
            script.extend_from_slice(data);
            script.extend_from_slice(&[0x88, 0xac]);
            Some(script)
        }
        1 => {
            // P2SH
            let mut script = vec![0xa9, 0x14];
            script.extend_from_slice(data);
            script.push(0x87);
            Some(script)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_amount() {
        let original = 12345;
        let compressed = compress_amount(original);
        let decompressed = decompress_amount(compressed);
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compress_decompress_script() {
        let script = vec![
            0x76, 0xa9, 0x14, 0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x7f, 0x8e, 0x9d, 0xac, 0xbb, 0xca,
            0xd9, 0xe8, 0xf7, 0x06, 0x15, 0x24, 0x33, 0x88, 0xac,
        ];
        let compressed = compress_script(&script).unwrap();
        let decompressed = decompress_script(&compressed, 0).unwrap();
        assert_eq!(script, decompressed);
    }
}
