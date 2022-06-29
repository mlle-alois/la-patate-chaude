use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub enum subscribeError {
    AlreadyRegistered, InvalidName
}
