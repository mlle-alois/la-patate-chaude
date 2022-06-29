use crate::Models::welcome::welcome;
use crate::Models::subscribe::subscribe;
use crate::Models::subscribeResult::subscribeResult;
use crate::Models::PublicLeaderBoard::PublicLeaderBoard;
use crate::Models::Challenge::Challenge;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::Models::PublicPlayer::PublicPlayer;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(welcome),
    Subscribe(subscribe),
    SubscribeResult(subscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
}