use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Empty {}

impl Default for Empty {
    fn default() -> Self {
        Empty {}
    }
}
