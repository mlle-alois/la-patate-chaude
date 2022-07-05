use serde::{Serialize, Deserialize};

#[derive(Debug , Serialize, Deserialize, PartialEq)]
pub struct Welcome {
    pub version:u8
}


