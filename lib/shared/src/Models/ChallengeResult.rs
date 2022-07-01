use serde::{Deserialize, Serialize};
use serde_json;

use crate::Models::ChallengeAnswer::ChallengeAnswer;
use crate::Models::ChallengeOutput::ChallengeOutput;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}