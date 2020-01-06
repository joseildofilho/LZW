use std::collections::HashMap;

pub trait LZW {
    fn encode(&self);
    fn new() -> Self;
    fn dict_ref(&self) -> &HashMap<String, String>;
}

pub struct LZWDict {
    dict: HashMap<String, String>
}

impl LZW for LZWDict {
    fn encode(&self) {
        println!("opá")
    }
    fn new() -> LZWDict {
        LZWDict {
            dict: HashMap::new()
        }
    }
    fn dict_ref(&self) -> &HashMap<String, String> {
        &&self.dict
    }
}
