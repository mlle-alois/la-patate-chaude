use crate::models::welcome::Welcome;
use crate::models::subscribe::Subscribe;
use crate::models::subscribe_result::SubscribeResult;
use crate::models::challenge::Challenge;
use serde::{Serialize, Deserialize};
use crate::models::challenge_result::ChallengeResult;
use crate::models::end_of_game::EndOfGame;
use crate::models::public_player::PublicPlayer;
use crate::models::round_summary::RoundSummary;

#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub enum Message {
    Excluded(String),
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}