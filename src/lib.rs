use std::collections::{HashMap, HashSet};
use bit_long_vec::BitLongVec;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LZWDict {
    index_lexeme: HashMap<usize, Vec<u64>>,
    lexeme_index: HashMap<Vec<u64>, usize>
}

impl LZWDict {
    fn new() -> Self {
        LZWDict {
            index_lexeme: HashMap::new(),
            lexeme_index: HashMap::new()
        }
    }
    fn insert(&mut self, key:usize, value:Vec<u64>) {
        &self.index_lexeme.insert(key, value.clone());
        &self.lexeme_index.insert(value, key);
    }
    fn fill_inital_alph(&mut self, k: u64) {
        for i in 0..(u64::pow(2, k as u32)) {
            self.insert(i as usize, vec![i]);
        }
    }
    fn get_index(&self, symbol: Vec<u64>) -> Option<&usize> {
        self.lexeme_index.get(&symbol)
    }
    fn get_lexeme(&self, index:usize) -> Option<&Vec<u64>> {
        self.index_lexeme.get(&index)
    }
    fn get_maximum_match(&mut self, slice:Vec<u64>) -> [usize; 2] {
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
                    self.insert(self.index_lexeme.len(), aux_slice.to_vec());
                    break;
                }
            }
        }
        [pointer, offset]
    }
}

pub trait LZW<'lifecycle> {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self;
    fn decode(msg: &'lifecycle LZWData) -> &[u8];
    fn codedmsg_ref(&self) -> &BitLongVec;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LZWData {
    dict    : LZWDict,
    codedmsg: BitLongVec,
    sizemsg: usize
}

fn concat_array_u8_u64(array: &[u8]) -> u64 {
    if array.len() > 8 {
        panic!("Eita");
    }
    let mut val: u64 = 0;
    for i in 0..array.len() {
        val |= (array[i] as u64) << (8 * i);
    }
    val
}

fn convert_array_u8_u64(array: &[u8]) -> Vec<u64> {
    let mut i:usize = 0;
    let mut vec: Vec<u64> = Vec::new();
    let mut offset = i;
    if array.len() == 0 {
        return vec
    }
    while i <= array.len() {
        if (offset + 8) > array.len() {
            vec.push(concat_array_u8_u64(&array[offset..]));
        } else {
            vec.push(concat_array_u8_u64(&array[offset..offset + 8]));
        }
        i += 8;
        offset = i
    }
    return vec;
}

impl<'lifecycle> LZW<'lifecycle> for LZWData {
    fn encode(data: &'lifecycle[u8], k: u8) -> Self {
        let l = data.len();
        let msg_data:Vec<u64> = convert_array_u8_u64(&data);
        let msg = BitLongVec::from_data(msg_data, l, k);
        let mut codedmsg:BitLongVec = BitLongVec::with_fixed_capacity(l, k);

        let mut dict = LZWDict::new();
        dict.fill_inital_alph(k as u64);

        let mut index  :usize = 0;
        let mut pointer:usize = 0;
        while pointer < l {
            let mut slice = Vec::new();
            for i in pointer..msg.capacity {
                slice.push(msg.get(i));
            }

            let result = dict.get_maximum_match(slice);
            println!("{:?}", result);
            codedmsg.set(index, result[0] as u64);
            pointer += result[1];
            index += 1;
            println!("{:?}", pointer);
        }
        LZWData {
            dict: dict,
            codedmsg: codedmsg,
            sizemsg: index
        }
    }
    fn decode(codedmsg: &'lifecycle LZWData) -> &[u8]{
        let mut data:Vec<u64> = Vec::new();
        let coded_data = &codedmsg.codedmsg;
        let dict = &codedmsg.dict;
        let mut next:&[u64] = dict.get_lexeme(coded_data.get(0) as usize).unwrap();
        data.push(next[0]);
        for index in 1..codedmsg.sizemsg {
            println!("Value: {}", coded_data.get(index));
            let lexeme = dict.get_lexeme(coded_data.get(index) as usize).unwrap();
   //         data.append(lexeme.to_vec());
   //         next = [next, lexeme].concat();
            println!("{:?}", lexeme);
        }
        return &[1]
    }
    fn codedmsg_ref(&self) -> &BitLongVec {
        &&self.codedmsg
    }
}
