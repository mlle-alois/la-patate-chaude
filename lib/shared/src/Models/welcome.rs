use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug , Serialize, Deserialize)]
pub struct welcome{
    pub version:u8
}


