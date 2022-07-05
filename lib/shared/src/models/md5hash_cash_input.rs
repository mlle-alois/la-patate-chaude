use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}