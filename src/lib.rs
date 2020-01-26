use std::collections::{HashMap, HashSet};
use bit_long_vec::BitLongVec;

#[derive(Debug)]
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
            &self.insert(index, vec![c]);
        }
    }
    fn get_index(&self, symbol: Vec<u8>) -> Option<&usize> {
        self.lexeme_index.get(&symbol)
    }
    fn get_maximum_match(&self, slice:&[u8]) -> usize {
        let mut aux:usize = 0;
        for i in 0..slice.len() {
            let aux_slice = &slice[..i+1];
            match self.get_index(aux_slice.to_vec()) {
                Some(x) => {
                    println!("Result: {}", x);
                    aux = *x;
                }, 
                None => {
                    println!("Found A Hole: {}", aux);
                    println!("Current Slice: {:?}", aux_slice);
                    break;
                }
            }
        }
        return aux
    }
}

pub trait LZW<'lifecycle> {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self;
    fn msg_ref(&self) -> &'lifecycle [u8];
    fn codedmsg_ref(&self) -> &BitLongVec;
}

pub struct LZWData<'lifecycle> {
    dict    : LZWDict,
    msg     : &'lifecycle [u8],
    codedmsg: BitLongVec
}

impl<'lifecycle> LZW<'lifecycle> for LZWData<'lifecycle> {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self {
        let mut codedmsg = BitLongVec::with_fixed_capacity(msg.len(), k);
        let mut dict = LZWDict::new();
        dict.find_n_fill_inital_alph(&msg);

        let mut index:usize = 0;
        while index < msg.len() {
            let slice: &[u8] = &msg[index..];
            codedmsg.set(index, dict.get_maximum_match(slice) as u64);
            index += 1;
        }

        LZWData {
            dict: dict,
            msg: msg,
            codedmsg: codedmsg
        }
    }
    fn msg_ref(&self) -> &'lifecycle [u8] {
        &&self.msg
    }
    fn codedmsg_ref(&self) -> &BitLongVec {
        &&self.codedmsg
    }
}
