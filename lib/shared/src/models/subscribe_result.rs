use crate::models::subscribe_error::SubscribeError;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}