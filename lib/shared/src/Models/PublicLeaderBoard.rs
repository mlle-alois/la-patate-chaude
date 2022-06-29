use crate::Models::PublicPlayer::PublicPlayer;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]

pub struct PublicLeaderBoard {
    players: Vec<PublicPlayer>
}