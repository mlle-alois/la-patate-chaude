pub struct public_player {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

impl public_player {
    fn name(&self) -> &str {
        &self.name
    }
}
