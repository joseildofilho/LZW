use lzw::{LZW, LZWData};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::collections::{HashMap};
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let path = "data/orl_faces";
    let folder = read_dir(&path)?;
    let mut map_folders:HashMap<String, Vec<DirEntry>> = HashMap::new();
    folder.for_each(|x| { map_folders.insert(x.unwrap().file_name().into_string().unwrap(), Vec::new());});
    map_folders.iter_mut().for_each(|tuple| {
        let dir  = read_dir(path.to_owned() + "/" + tuple.0).unwrap();
        dir.for_each(|x| tuple.1.push(x.unwrap()));
    });

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
            test_data.insert(class.to_string(), file_contents_test);

            elements.iter().for_each(|file_path| {
                let path = file_path.path();
                let mut file = File::open(path).unwrap();
                file.read_to_end(&mut file_contents_training);
            });

            training_data.insert(class.to_string(), file_contents_training);
        };
    });

    for k in 9..=16 {
       let mut now = SystemTime::now();
       let mut trained_dicts: HashMap<String, LZWData> = HashMap::new();
       training_data.iter().for_each(|tuple| {
           let label = tuple.0;
           let data  = tuple.1;
        
           let compressor: LZWData = LZW::encode(&data, k);
           //compressor.dict.set_maximum(1);
           trained_dicts.insert(label.clone(), compressor);
       });
       println!();
       let mut hits = 0;
       test_data.iter().for_each(|tuple| {
           let label = tuple.0;
           let test_data  = tuple.1;
        
           let mut min: usize = usize::max_value();
           let mut label_min: String = String::new();
           trained_dicts.iter().for_each(|trained_tuple| {
               let trained_dict_label = trained_tuple.0;
               let trained_dict = trained_tuple.1;
               let x: LZWData = LZW::encode_with_dict(test_data, k, trained_dict.dict.to_owned());
               if x.msg_size() < min {
                   min = x.msg_size();
                   label_min = trained_dict_label.to_owned();
               }
           });
           let hit = *label == label_min;
           if hit {
               hits += 1
           }
       });
    
       println!("{}/{} k:{}, elapsed: {}", hits, test_data.len(), k, now.elapsed().unwrap().as_secs_f32());
    }
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