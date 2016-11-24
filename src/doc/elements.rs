use std::cmp::Eq;
use std::hash::Hash;

#[derive(Hash, PartialEq)]
pub enum Elements {
    Text {
        text: Vec<u8>
    },
    BoldText {
        text: Vec<u8>
    }
}

impl Eq for Elements {}
