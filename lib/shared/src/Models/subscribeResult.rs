use crate::Models::subscribeError::subscribeError;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub enum subscribeResult {
    Ok,
    Err(subscribeError),
}