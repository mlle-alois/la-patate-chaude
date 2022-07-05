use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicPlayer {
    pub name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

impl PublicPlayer {
    fn name(&self) -> &str {
        &self.name
    }
}
