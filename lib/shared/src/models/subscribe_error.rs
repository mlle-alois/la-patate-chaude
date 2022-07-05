use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SubscribeError {
    AlreadyRegistered, InvalidName
}
