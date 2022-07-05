use serde::{Deserialize, Serialize};

use crate::models::challenge_answer::ChallengeAnswer;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}