use std::collections::HashMap;
use bit_long_vec::BitLongVec;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LZWDict {
    index_lexeme: HashMap<usize, Vec<u8>>,
    lexeme_index: HashMap<Vec<u8>, usize>,
    size   : usize,
    maximum: usize
}

impl LZWDict {
    fn new() -> Self {
        LZWDict {
            index_lexeme: HashMap::new(),
            lexeme_index: HashMap::new(),
            size: 0,
            maximum: 0
        }
    }

    fn insert(&mut self, key:usize, value:Vec<u8>) {
        &self.index_lexeme.insert(key, value.clone());
        &self.lexeme_index.insert(value, key);
        self.size += 1;
    }

    pub fn set_maximum(&mut self, k: usize) {
        self.maximum = k
    }

    fn fill_inital_alph(&mut self) {
        for i in 0..(u16::pow(2, 8)) {
            self.insert(i as usize, vec![i as u8]);
        }
    }

    fn get_index(&self, symbol: Vec<u8>) -> Option<&usize> {
        self.lexeme_index.get(&symbol)
    }

    fn get_lexeme(&self, index:usize) -> Option<&Vec<u8>> {
        self.index_lexeme.get(&index)
    }

    fn get_maximum_match(&mut self, slice:&[u8]) -> [usize; 2] {
        let mut pointer:usize = 0;
        let mut offset :usize = 0;
        for i in 0..slice.len() {
            let aux_slice = &slice[..i+1];
            match self.get_index(aux_slice.to_vec()) {
                Some(x) => {
                    offset = aux_slice.len();
                    pointer = *x;
                }, 

                None => {
                    if self.maximum > self.size {
                        self.insert(self.index_lexeme.len(), aux_slice.to_vec());
                    }
                    break;
                }
            }
        }

        [pointer, offset]
    }
}

pub trait LZW<'lifecycle> {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self;
    fn encode_with_dict(msg: &'lifecycle[u8], k: u8, dict: LZWDict) -> Self;
    fn decode(msg: &'lifecycle LZWData) -> Vec<u8>;
    fn save_bin_file(&self, path: &str);
    fn decode_bin_file(path: &str, k: u8) -> Vec<u8>;
    fn codedmsg_ref(&self) -> &BitLongVec;
    fn msg_size(&self) -> usize;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LZWData {
    pub dict: LZWDict,
    codedmsg: BitLongVec,
    sizemsg: usize
}

impl<'lifecycle> LZW<'lifecycle> for LZWData {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self {

        let mut dict = LZWDict::new();
        dict.fill_inital_alph();
        dict.set_maximum(usize::pow(2,k as u32));
        Self::encode_with_dict(msg, k, dict)
    }

    fn encode_with_dict(msg: &'lifecycle[u8], k: u8, dict: LZWDict) -> Self {
        let mut dict = dict;
        let mut codedmsg:BitLongVec = BitLongVec::with_fixed_capacity(msg.len(),k);
        f64::log(10.0, 2.0) as u8;
        let mut index  :usize = 0;
        let mut pointer:usize = 0;
        while pointer < msg.len() {
            let slice = &msg[pointer..];

            let result = dict.get_maximum_match(slice);
            codedmsg.set(index, result[0] as u64);
            pointer += result[1];
            index += 1;
        };
        Self {
            dict: dict,
            codedmsg: codedmsg,
            sizemsg: index
        }
    }

    fn decode(codedmsg: &'lifecycle LZWData) -> Vec<u8> {
        let mut data:Vec<u8> = Vec::new();
        let coded_data = &codedmsg.codedmsg;
        let dict = &codedmsg.dict;
        let next:&[u8] = dict.get_lexeme(coded_data.get(0) as usize).unwrap();
        data.push(next[0]);
        for index in 1..codedmsg.sizemsg {
            let lexeme = dict.get_lexeme(coded_data.get(index) as usize).unwrap();
            data.extend_from_slice(lexeme.as_slice());
        }

        return data
    }

    fn codedmsg_ref(&self) -> &BitLongVec {
        &&self.codedmsg
    }

    fn msg_size(&self) -> usize {
        return self.sizemsg
    }

    fn save_bin_file(&self, path: &str) {
        let msg = &self.codedmsg;
        let mut file = File::create(path).unwrap();
        for long in &msg.data {
            if *long != 0 {
                file.write_all(&long.to_le_bytes());
            }
        }
    }

    fn decode_bin_file(path: &str, k: u8) -> Vec<u8> {
        let mut file = File::open(path).unwrap();
        let mut decoded: Vec<u8> = Vec::new();
        let mut values: Vec<u64> = Vec::new();
        let mut aux: Vec<u8> = Vec::new();

        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer);
        for i in (0..buffer.len()).step_by(8) {
            let mut array: [u8; 8] = [0; 8];
            array.copy_from_slice(&buffer[i..i+8]);
            let val = u64::from_le_bytes(array);
            values.push(val);
        }
        let values = BitLongVec::from_data(values.clone(), (buffer.len() * 8 - 1) / k as usize, k);

        let mut dict: LZWDict = LZWDict::new();
        dict.fill_inital_alph();
        dict.set_maximum(usize::pow(2, k as u32));
        let mut next_code = 256;
        for i in 0..values.capacity {
            let code = values.get(i) as usize;
            if dict.get_lexeme(code).is_none() {
                aux.push(aux[0]);
                dict.insert(code, aux.clone());
                aux.pop();
            }
            
            let lexeme = dict.get_lexeme(code as usize).unwrap();
            decoded.extend_from_slice(lexeme.as_slice());
            if aux.len() != 0 {
                aux.push(lexeme[0]);
                dict.insert(next_code, aux.clone());
                aux.pop();
                next_code += 1;
            }

            aux = dict.get_lexeme(code as usize).unwrap().to_vec();
        }
        
        while decoded[decoded.len() - 1] == 0 {
            decoded.pop();
        }

        return decoded
    }
}
