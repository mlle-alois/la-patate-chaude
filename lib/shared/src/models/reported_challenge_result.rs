use serde::{Serialize, Deserialize};
use crate::models::challenge_value::ChallengeValue;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReportedChallengeResult {
    name :String,
    value: ChallengeValue
}