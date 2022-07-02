use crate::Models::welcome::welcome;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug , Serialize, Deserialize, PartialEq)]
pub struct subscribe{
    pub name:String
}