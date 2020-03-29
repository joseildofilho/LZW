use lzw::{LZW, LZWData};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let path = "data/orl_faces";
    let folder = read_dir(&path)?;
    let mut map_folders:HashMap<String, Vec<DirEntry>> = HashMap::new();
    folder.for_each(|x| { map_folders.insert(x.unwrap().file_name().into_string().unwrap(), Vec::new());});
    map_folders.iter_mut().for_each(|tuple| {
        let dir  = read_dir(path.to_owned() + "/" + tuple.0).unwrap();
        dir.for_each(|x| tuple.1.push(x.unwrap()));
    });
    println!("{:#?}", map_folders);

    let mut training_data: HashMap<String, Vec<u8>> = HashMap::new();
    let mut test_data:     HashMap<String, Vec<u8>> = HashMap::new();

    // Loads the data to these Maps above.
    map_folders.iter().for_each(|tuple| {
        let class = tuple.0;
        let files_path = tuple.1;
        if let Some((last, elements)) = files_path.split_last() {
            let mut file_contents_training = vec![];
            let mut file_contents_test = vec![];
            let path = last.path();
            let mut file = File::open(path).unwrap();
            file.read_to_end(&mut file_contents_test);
            test_data.insert(last.file_name().into_string().unwrap(), file_contents_test);

            elements.iter().for_each(|file_path| {
                let path = file_path.path();
                let mut file = File::open(path).unwrap();
                file.read_to_end(&mut file_contents_training);
            });

            training_data.insert(last.file_name().into_string().unwrap(), file_contents_training);
        };
    });

    let mut files_dicts: HashMap<String, LZWData> = HashMap::new();
    training_data.iter().for_each(|tuple| {
        let label = tuple.0;
        let data  = tuple.1;

        let compressor: LZWData = LZW::encode(&data, 9);
        files_dicts.insert(label.clone(), compressor);
    });

    

    Ok(())
}
    /*
    let files = ["corpus16MB.txt"/*, "mapa.mp4"*/];
    for f in &files {
        println!("File:{}\n", f);
        for k in 9..=9 {
            let mut file = File::open(f)?;
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents);
            println!("K:{}", k);
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
            
            let s = serde_yaml::to_string(&compressor).unwrap().to_string();
            let mut file_ = File::create("data.txt")?;
            write!(file_, "{}", s)?;
            /*let aux: Vec<&str> = f.split('.').collect();
            let filename = format!("{}_k{}.fkzip", aux[0], k);
            compressor.save_bin_file(&filename.clone());
            LZWData::decode_bin_file(&filename, k);
            now = SystemTime::now();
            let decoded = LZWData::decode_bin_file(&filename, k);
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("Decode time:{}\n", elapsed.as_secs_f32());
                },

                Err(e) => {
                    println!("Error:{:?}", e);
                }
            }
            let mut decoded_file = File::create(format!("decompressed_{}_k{}.{}", aux[0], k, aux[1]))?;
            decoded_file.write_all(decoded.as_slice());
            */
        }
    }*/