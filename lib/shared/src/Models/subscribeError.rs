use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum subscribeError {
    AlreadyRegistered, InvalidName
}
