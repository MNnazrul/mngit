pub fn u32_to_be_bytes(x: u32) -> [u8; 4] {
    x.to_be_bytes()
}

pub fn u32_from_be_bytes(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

pub fn rotl(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

pub fn to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);

    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0F) as usize] as char);
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotl_basic() {
        assert_eq!(rotl(0x0000_0001, 1), 0x0000_0002);
        assert_eq!(rotl(0x8000_0000, 1), 0x0000_0001);
    }

    #[test]
    fn test_be_roundtrip() {
        let x = 0x1234_5678u32;
        let b = u32_to_be_bytes(x);
        assert_eq!(b, [0x12, 0x34, 0x56, 0x78]);
        assert_eq!(u32_from_be_bytes(b), x);
    }

    #[test]
    fn test_hex() {
        let bytes = [0xDA, 0x39, 0xA3];
        assert_eq!(to_hex_lower(&bytes), "da39a3");
    }
}
