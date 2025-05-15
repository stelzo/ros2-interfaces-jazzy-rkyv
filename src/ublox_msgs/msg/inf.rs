use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Inf {
    pub str: Vec<i8>,
}

impl Inf {
    pub const CLASS_ID: u8 = 4;
}

impl Default for Inf {
    fn default() -> Self {
        Inf { str: Vec::new() }
    }
}
