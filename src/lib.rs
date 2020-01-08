use std::collections::{HashMap, HashSet};
use bit_long_vec::BitLongVec;

pub trait LZW<'lifecycle> {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self;
    fn dict_ref(&self) -> &HashMap<usize, Vec<u8>>;
    fn msg_ref(&self) -> &'lifecycle [u8];
    fn codedmsg_ref(&self) -> &BitLongVec;
}

pub struct LZWData<'lifecycle> {
    dict    : HashMap<usize, Vec<u8>>,
    msg     : &'lifecycle [u8],
    codedmsg: BitLongVec
}

fn create_inital_dict(msg: &[u8]) -> HashMap<usize, Vec<u8>> {
    if msg.len() == 0 {
        return HashMap::new();
    }
    let mut set:HashSet<u8> = HashSet::new();
    for c in msg.iter() {
        set.insert(*c);
    }
    let mut vec:Vec<u8> = Vec::with_capacity(set.len());
    for c in set.drain() {
        vec.push(c);
    }
    vec.sort();
    let mut map:HashMap<usize, Vec<u8>> = HashMap::new();
    for (index, c) in vec.drain(0..).enumerate() {
        map.insert(index, vec![c]);
    }
    map
}

impl<'lifecycle> LZW<'lifecycle> for LZWData<'lifecycle> {
    fn encode(msg: &'lifecycle[u8], k: u8) -> Self {
        let dict = create_inital_dict(msg);
        println!("{:?}", dict);
        LZWData {
            dict: dict,
            msg: msg,
            codedmsg: BitLongVec::with_fixed_capacity(msg.len(), k)
        }
    }
    fn dict_ref(&self) -> &HashMap<usize, Vec<u8>> {
        &&self.dict
    }
    fn msg_ref(&self) -> &'lifecycle [u8] {
        &&self.msg
    }
    fn codedmsg_ref(&self) -> &BitLongVec {
        &&self.codedmsg
    }
}
