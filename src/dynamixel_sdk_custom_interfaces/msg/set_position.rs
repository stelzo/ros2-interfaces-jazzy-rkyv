use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPosition {
    pub id: u8,
    pub position: i32,
}

impl Default for SetPosition {
    fn default() -> Self {
        SetPosition { id: 0, position: 0 }
    }
}
