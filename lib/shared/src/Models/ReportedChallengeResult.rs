use serde::{Serialize, Deserialize};
use serde_json;
use crate::Models::ChallengeValue::ChallengeValue;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReportedChallengeResult {
    name :String,
    value: ChallengeValue
}