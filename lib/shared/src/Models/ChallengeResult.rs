use serde::{Deserialize, Serialize};
use serde_json;

use crate::Models::ChallengeAnswer::ChallengeAnswer;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}