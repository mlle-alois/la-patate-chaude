use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeInput {
    complexity: i32,
    message: String
}

