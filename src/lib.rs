use std::collections::{HashMap, HashSet};
use bit_long_vec::BitLongVec;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LZWDict {
    index_lexeme: HashMap<usize, Vec<u8>>,
    lexeme_index: HashMap<Vec<u8>, usize>
}

impl LZWDict {
    fn new() -> Self {
        LZWDict {
            index_lexeme: HashMap::new(),
            lexeme_index: HashMap::new()
        }
    }
    fn insert(&mut self, key:usize, value:Vec<u8>) {
        &self.index_lexeme.insert(key, value.clone());
        &self.lexeme_index.insert(value, key);
    }
    fn find_n_fill_inital_alph(&mut self, msg:&[u8]) {
        let mut set:HashSet<u8> = HashSet::new();
        for c in msg.iter() {
            set.insert(*c);
        }
        let mut vec:Vec<u8> = Vec::with_capacity(set.len());
        for c in set.drain() {
            vec.push(c);
        }
        vec.sort();
        for (index, c) in vec.drain(0..).enumerate() {
            println!("Inserting: {:?}", c);
            &self.insert(index, vec![c]);
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
    let mut i:usize = 8;
    let mut vec: Vec<u64> = Vec::new();
    while i <= array.len() {
        vec.push(concat_array_u8_u64(&array[(i - 8)..i]));
        if (i + 8) > array.len() {
            vec.push(concat_array_u8_u64(&array[i..]));
        }
        i += 8;
    }
    return vec;
}

impl<'lifecycle> LZW<'lifecycle> for LZWData {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self {
        let mut codedmsg = BitLongVec::with_fixed_capacity(msg.len(), k);
        let mut dict = LZWDict::new();
        dict.find_n_fill_inital_alph(&msg);
        let initial_dict = dict.clone();

        let mut index  :usize = 0;
        let mut pointer:usize = 0;
        while pointer < msg.len() {
            let slice: &[u8] = &msg[pointer..];
            let result = dict.get_maximum_match(slice);
            codedmsg.set(index, result[0] as u64);
            pointer += result[1];
            index += 1
        }
        LZWData {
            dict: initial_dict,
            codedmsg: codedmsg,
            sizemsg: index
        }
    }
    fn decode(codedmsg: &'lifecycle LZWData) -> &[u8]{
        let mut data:Vec<u8> = Vec::new();
        let coded_data = &codedmsg.codedmsg;
        let dict = &codedmsg.dict;
        let mut next:&[u8] = dict.get_lexeme(coded_data.get(0) as usize).unwrap();
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
