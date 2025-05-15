use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AllowedCollisionEntry {
    pub enabled: Vec<bool>,
}

impl Default for AllowedCollisionEntry {
    fn default() -> Self {
        AllowedCollisionEntry {
            enabled: Vec::new(),
        }
    }
}
