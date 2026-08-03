use std::collections::{HashMap};

// DATA ---------------------------------------------------------------------
#[derive(Clone)]
pub enum Data { // TODO: Suppr pub
    Str(Vec<u8>),
    Hash(HashMap<Vec<u8>, Vec<u8>>),
}
impl Data {
    pub fn data_type(&self) -> DataType {
        match self {
            Data::Str(_)  => DataType::Str,
            Data::Hash(_) => DataType::Hash,
        }
    }

    pub fn as_hash_mut(&mut self) -> Option<&mut HashMap<Vec<u8>, Vec<u8>>> {
        match self {
            Data::Hash(h) => Some(h),
            _ => None,
        }
    }
    pub fn as_hash(&self) -> Option<&HashMap<Vec<u8>, Vec<u8>>> {
        match self {
            Data::Hash(h) => Some(h),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<String> {
        match self {
            Data::Str(bytes) => Some(String::from_utf8_lossy(bytes).to_string()),
            _ => None,
        }
    }

    pub fn to_str(&self) -> Result<String, &'static str> {
        match self {
            Data::Str(bytes) => {
                    match std::str::from_utf8(bytes) {
                        Ok(s) => Ok(s.to_string()),
                        _ => Err("ERROR: invalid string")
                    }
                },
            Data::Hash(h) => {
                    let mut str : String = "{\n".to_string();
                    for (key, value) in h.into_iter() {
                        str = format!("{}\t{} : {}\n", str, String::from_utf8_lossy(key), String::from_utf8_lossy(value));
                    }
                    str.push_str("}");
                    Ok(str)
                }, 
            _ => Err("ERROR: type not handled on to_str"),
        }
    }
}



// VALUE ---------------------------------------------------------------------

#[derive(Clone)]
pub struct Value {
    pub data : Data,
}

impl Value {
    pub fn data(&self) -> &Data { &self.data }
    pub fn data_type(&self) -> DataType { self.data.data_type() }

    pub fn new_hash() -> Value { Value { data : Data::Hash(HashMap::new()) } }

}

impl From<Data> for Value {
    fn from(data: Data) -> Self { Value { data } }
}
impl From<String> for Value {
    fn from(s: String) -> Self { Value { data: Data::Str(s.into_bytes()) } }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self { Value { data: Data::Str(s.as_bytes().to_vec()) } }
}
impl From< HashMap< Vec<u8>, Vec<u8> > > for Value {
    fn from(h: HashMap<Vec<u8>, Vec<u8>>) -> Self { Value { data: Data::Hash(h) } }
}



// DATATYPE ---------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DataType {
    Str,
    Hash,
}



