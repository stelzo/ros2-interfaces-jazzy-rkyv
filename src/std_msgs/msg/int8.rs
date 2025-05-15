use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int8 {
    pub data: i8,
}

impl Default for Int8 {
    fn default() -> Self {
        Int8 { data: 0 }
    }
}
