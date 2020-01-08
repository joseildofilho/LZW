#[cfg(test)]
mod tests {
    use lzw::{LZW, LZWData};
    use std::collections::HashMap;
    #[test]
    fn test_lwz_empty() {
        let msg: &[u8] = "".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);

        let dict: HashMap<usize, Vec<u8>> = HashMap::new();

        assert_eq!(compressor.dict_ref(), &dict);
        assert_eq!(compressor.msg_ref(), msg);
    }
    #[test]
    fn test_lwz_one() {
        let msg: &[u8] = "a".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut dict: HashMap<usize, Vec<u8>> = HashMap::new();
        dict.insert(0, "a".as_bytes().to_vec());

        assert_eq!(compressor.dict_ref(), &dict);
        assert_eq!(compressor.msg_ref(), msg);
    }
}
