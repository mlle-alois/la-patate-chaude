use serde::{Deserialize, Serialize};
use serde_json;

use crate::Models::ChallengeInput::ChallengeInput;
use crate::Models::ChallengeOutput::ChallengeOutput;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    ChallengeName(ChallengeOutput)
}