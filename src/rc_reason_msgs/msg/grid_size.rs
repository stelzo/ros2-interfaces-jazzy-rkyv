use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GridSize {
    pub x: u32,
    pub y: u32,
}

impl Default for GridSize {
    fn default() -> Self {
        GridSize { x: 0, y: 0 }
    }
}
