use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int16 {
    pub data: i16,
}

impl Default for Int16 {
    fn default() -> Self {
        Int16 { data: 0 }
    }
}
