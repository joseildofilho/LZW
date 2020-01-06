#[cfg(test)]
mod tests {
    use lzw::{LZW, LZWDict};
    use std::collections::HashMap;
    #[test]
    fn test_new_lwz() {
        let compressor:LZWDict = LZW::new();
        assert_eq!(compressor.dict_ref(), &HashMap::new());
    }
}
