pub fn decode(str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(str.to_lowercase().trim_start_matches("0x"))
}

pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let mut s = String::from("0x");
    s.push_str(hex::encode(data).as_str());
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_without_prefix() {
        assert_eq!(decode("68656c6c6f").unwrap(), "hello".as_bytes());
    }

    #[test]
    fn test_decode_with_prefix() {
        assert_eq!(decode("0x68656c6c6f").unwrap(), "hello".as_bytes());
    }

    #[test]
    fn test_decode_with_prefix_upper_case() {
        assert_eq!(decode("0X68656C6C6F").unwrap(), "hello".as_bytes());
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("hello"), "0x68656c6c6f");
    }
}
