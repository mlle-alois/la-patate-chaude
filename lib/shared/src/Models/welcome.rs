use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug , Serialize, Deserialize, PartialEq)]
pub struct welcome{
    pub version:u8
}


