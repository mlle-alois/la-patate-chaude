use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeOutput {
    pub seed: i32,
    pub hashcode: String
}

