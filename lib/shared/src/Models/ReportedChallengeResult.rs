use serde::{Serialize, Deserialize};
use serde_json;
use crate::Models::ChallengeValue::ChallengeValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name :String,
    value: ChallengeValue
}