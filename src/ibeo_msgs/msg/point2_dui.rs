use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Point2Dui {
    pub x: u16,
    pub y: u16,
}

impl Default for Point2Dui {
    fn default() -> Self {
        Point2Dui { x: 0, y: 0 }
    }
}
