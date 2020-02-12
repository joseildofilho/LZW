#[cfg(test)]
mod tests {
    use lzw::{LZW, LZWData};
    use std::collections::HashMap;
    use bit_long_vec::BitLongVec;
    use serde_json::Value;
    #[test]
    fn test_lwz_empty() {
        let msg: &[u8] = "".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);

        let bits = BitLongVec::with_fixed_capacity(msg.len(), k);

        assert_eq!(compressor.codedmsg_ref(), &bits)
        
    }
    #[test]
    fn test_lwz_one() {
        let msg: &[u8] = "a".as_bytes();
        let k: u8 = 9;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(0, 97);

        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_two_new() {
        let msg: &[u8] = "ab".as_bytes();
        let k: u8 = 8;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(2, k);
        bits.set(0, 97);
        bits.set(1, 98);

        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_three_new() {
        let msg: &[u8] = "abc".as_bytes();
        let k: u8 = 8;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(0, 97);
        bits.set(1, 98);
        bits.set(2,99);

        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_repeat() {
        let msg: &[u8] = "abb".as_bytes();
        let k: u8 = 8;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(0,97);
        bits.set(1,98);
        bits.set(2,98);

        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_repeat_2() {
        let msg: &[u8] = "abbb".as_bytes();
        let k: u8 = 9;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(0,97);
        bits.set(1,98);
        bits.set(2,257);

        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_repeat_3() {
        let msg: &[u8] = "abbbb".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(0,97);
        bits.set(1,98);
        bits.set(2,257);
        bits.set(3,98);

        assert_eq!(compressor.codedmsg_ref(), &bits)
    }

    #[test]
    fn test_lzw_abracadabra() {
        let msg: &[u8] = "abracadabra".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);

        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(0,97);
        bits.set(1,98);
        bits.set(2,114);
        bits.set(3,97);
        bits.set(4,99);
        bits.set(5,97);
        bits.set(6,100);
        bits.set(7,256);
        bits.set(8, 258);

        assert_eq!(compressor.codedmsg_ref(), &bits);
    }
    #[test]
    fn test_lzw_serialize() {
        let msg: &[u8] = "abb".as_bytes();
        let k: u8 = 8;
        let compressor:LZWData = LZW::encode(msg, k);

        let serialized = serde_json::to_string(&compressor);
        println!("serialized = {:?}", serialized);
    }
    #[test]
    fn test_lzw_decode() {
        
    }
}
