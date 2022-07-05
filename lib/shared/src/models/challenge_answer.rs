use serde::{Deserialize, Serialize};
use crate::models::md5hash_cash_output::MD5HashCashOutput;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}