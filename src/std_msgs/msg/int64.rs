use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int64 {
    pub data: i64,
}

impl Default for Int64 {
    fn default() -> Self {
        Int64 { data: 0 }
    }
}
