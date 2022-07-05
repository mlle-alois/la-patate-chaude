use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub seed: u64,
    // hashcode found using seed + message
    pub hashcode: String,
}