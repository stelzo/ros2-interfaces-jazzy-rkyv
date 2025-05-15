use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Char {
    pub data: i8,
}

impl Default for Char {
    fn default() -> Self {
        Char { data: 0 }
    }
}
