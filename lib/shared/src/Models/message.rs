use crate::Models::welcome::welcome;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(welcome)
}