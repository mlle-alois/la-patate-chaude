use serde::{Serialize, Deserialize};
#[derive(Debug , Serialize, Deserialize, PartialEq)]
pub struct Subscribe {
    pub name:String
}