use crate::Models::welcome::welcome;

#[derive(Debug)]
pub enum message{
    Hello,
    Welcome(welcome)
}