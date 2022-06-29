use crate::Models::welcome::welcome;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug , Serialize, Deserialize)]
pub struct Subscribe{
    pub name:String
}