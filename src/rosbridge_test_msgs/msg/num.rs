use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Num {
    pub num: i64,
}

impl Default for Num {
    fn default() -> Self {
        Num { num: 0 }
    }
}
