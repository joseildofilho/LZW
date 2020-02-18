use lzw::{LZW, LZWData};
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let files = ["corpus16MB.txt", "mapa.mp4"];
    for f in &files {
        println!("File:{}", f);
        for k in 9..=16 {
            let mut file = File::open(f)?;
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents);
            println!("K:{}", k);
            //println!("Encoding...");
            let mut now = SystemTime::now();
            let compressor:LZWData = LZW::encode(&file_contents, k);
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("Encode time:{}", elapsed.as_secs_f32());
                },

                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }

            println!("Original file size:{}", file_contents.len() as f32 / (1024 * 1024) as f32);
            println!("Compressed file size:{}", compressor.msg_size() as f32 / (1024 * 1024) as f32);
            
            let s = serde_yaml::to_string(&compressor);
            let aux: Vec<&str> = f.split('.').collect();
            let filename = format!("{}_k{}.fkzip", aux[0], k);
            //let mut compressed = File::create(filename.clone())?;
            //compressed.write_all(s.unwrap().as_bytes());
            compressor.save_bin_file(&filename.clone());
            //return Ok(());
            now = SystemTime::now();
            LZWData::decode_bin_file(&filename, k);
            //let decoded = LZWData::decode(&serialized);
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("Decode time:{}\n", elapsed.as_secs_f32());
                },

                Err(e) => {
                    println!("Error:{:?}", e);
                }
            }
            
            //return Ok(());
            /*
            //println!("Encoding done.");
            //println!("Begin decoding...");
            let mut encoded_file = File::open(filename)?;
            //println!("Creating dictionary...");
            let serialized:LZWData = serde_yaml::from_reader(&encoded_file).unwrap();
            //println!("Decoding...");
            
            
            let mut decoded_file = File::create(format!("decompressed_{}_k{}.{}", aux[0], k, aux[1]))?;
            decoded_file.write_all(decoded.as_slice());*/
        }
    }
    Ok(())
}
