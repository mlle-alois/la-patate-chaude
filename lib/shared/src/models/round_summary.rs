use serde::{Deserialize, Serialize};
use crate::models::reported_challenge_result::ReportedChallengeResult;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>,
}