use crate::Models::MD5HashCashInput::MD5HashCashInput;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput)
}
/*
pub enum Challenge {
    ChallengeName(ChallengeInput)
}

 */