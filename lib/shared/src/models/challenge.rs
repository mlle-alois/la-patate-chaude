use crate::models::md5hash_cash_input::MD5HashCashInput;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput)
}
