use serde::{Deserialize, Serialize};
use crate::models::public_player::PublicPlayer;

#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}