use serde::{Deserialize, Serialize};
use serde_json;

use crate::Models::ReportedChallengeResult::ReportedChallengeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>,
}