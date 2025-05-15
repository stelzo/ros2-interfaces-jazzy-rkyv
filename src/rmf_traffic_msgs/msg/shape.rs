use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Shape {
    pub r#type: u8,
    pub index: u8,
}

impl Shape {
    pub const NONE: u8 = 0;
    pub const BOX: u8 = 1;
    pub const CIRCLE: u8 = 2;
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            r#type: 0,
            index: 0,
        }
    }
}
