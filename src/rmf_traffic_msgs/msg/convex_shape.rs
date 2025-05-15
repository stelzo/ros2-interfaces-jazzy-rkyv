use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConvexShape {
    pub r#type: u8,
    pub index: u8,
}

impl ConvexShape {
    pub const NONE: u8 = 0;
    pub const BOX: u8 = 1;
    pub const CIRCLE: u8 = 2;
}

impl Default for ConvexShape {
    fn default() -> Self {
        ConvexShape {
            r#type: 0,
            index: 0,
        }
    }
}
