use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Synchronize {}

impl Default for Synchronize {
    fn default() -> Self {
        Synchronize {}
    }
}
