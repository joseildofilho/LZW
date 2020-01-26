#[cfg(test)]
mod tests {
    use lzw::{LZW, LZWData};
    use std::collections::HashMap;
    use bit_long_vec::BitLongVec;
    #[test]
    fn test_lwz_empty() {
        let msg: &[u8] = "".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);

        let bits = BitLongVec::with_fixed_capacity(msg.len(), k);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
        
    }
    #[test]
    fn test_lwz_one() {
        let msg: &[u8] = "a".as_bytes();
        let k: u8 = 10;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_two_new() {
        let msg: &[u8] = "ab".as_bytes();
        let k: u8 = 3;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(1,1);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_three_new() {
        let msg: &[u8] = "abc".as_bytes();
        let k: u8 = 3;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(1,1);
        bits.set(2,2);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_repeat() {
        let msg: &[u8] = "abb".as_bytes();
        let k: u8 = 4;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(1,1);
        bits.set(2,1);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_repeat_2() {
        let msg: &[u8] = "abbb".as_bytes();
        let k: u8 = 4;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(1,1);
        bits.set(2,3);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
    }
    #[test]
    fn test_lwz_repeat_3() {
        let msg: &[u8] = "abbbb".as_bytes();
        let k: u8 = 4;
        let compressor:LZWData = LZW::encode(msg, k);
        
        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(1,1);
        bits.set(2,3);
        bits.set(3,1);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits)
    }

    #[test]
    fn test_lzw_abracadabra() {
        let msg: &[u8] = "abracadabra".as_bytes();
        let k: u8 = 4;
        let compressor:LZWData = LZW::encode(msg, k);

        let mut bits = BitLongVec::with_fixed_capacity(msg.len(), k);
        bits.set(1, 1);
        bits.set(2, 4);
        bits.set(3, 0);
        bits.set(4, 2);
        bits.set(5, 0);
        bits.set(6, 3);
        bits.set(7, 5);
        bits.set(8, 7);

        assert_eq!(compressor.msg_ref(), msg);
        assert_eq!(compressor.codedmsg_ref(), &bits);

    }
}
