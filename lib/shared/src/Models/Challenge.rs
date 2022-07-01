use crate::Models::ChallengeInput::ChallengeInput;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(ChallengeInput)
}
/*
pub enum Challenge {
    ChallengeName(ChallengeInput)
}

 */