use serde::{Deserialize, Serialize};
use serde_json;
use crate::Models::PublicLeaderBoard::PublicLeaderBoard;

use crate::Models::ReportedChallengeResult::ReportedChallengeResult;

#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}