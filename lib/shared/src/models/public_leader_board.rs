use crate::models::public_player::PublicPlayer;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]

pub struct PublicLeaderBoard {
    pub public_leader_board: Vec<PublicPlayer>
}