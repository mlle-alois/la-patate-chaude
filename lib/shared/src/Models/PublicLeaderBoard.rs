use crate::Models::PublicPlayer::PublicPlayer;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize, PartialEq)]

pub struct PublicLeaderBoard {
    pub public_leader_board: Vec<PublicPlayer>
}