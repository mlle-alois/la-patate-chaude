use crate::Models::welcome::welcome;
use crate::Models::subscribe::subscribe;
use crate::Models::subscribeResult::subscribeResult;
use crate::Models::PublicLeaderBoard::PublicLeaderBoard;
use crate::Models::Challenge::Challenge;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::Models::ChallengeResult::ChallengeResult;
use crate::Models::EndOfGame::EndOfGame;
use crate::Models::PublicPlayer::PublicPlayer;
use crate::Models::RoundSummary::RoundSummary;

#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub enum Message {
    Excluded(String),
    Hello,
    Welcome(welcome),
    Subscribe(subscribe),
    SubscribeResult(subscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}