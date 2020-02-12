use lzw::{LZW, LZWData};
use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let msg: &[u8] = "abracadabra".as_bytes();
    let k: u8 = 8; 
    let compressor:LZWData = LZW::encode(msg, k);

    let mut file = File::open("test.comp")?;
    let serialized:LZWData = serde_yaml::from_reader(&file).unwrap(); 
    println!("{:?}", LZWData::decode(&serialized));
    Ok(())
}
