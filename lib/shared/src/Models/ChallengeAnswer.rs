use serde::{Deserialize, Serialize};
use serde_json;

use crate::Models::MD5HashCashOutput::MD5HashCashOutput;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}