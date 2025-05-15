use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int32 {
    pub data: i32,
}

impl Default for Int32 {
    fn default() -> Self {
        Int32 { data: 0 }
    }
}
