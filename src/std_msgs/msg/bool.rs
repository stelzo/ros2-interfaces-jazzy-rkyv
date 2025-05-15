use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Bool {
    pub data: bool,
}

impl Default for Bool {
    fn default() -> Self {
        Bool { data: false }
    }
}
