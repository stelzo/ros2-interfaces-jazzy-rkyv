use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IrSourceInfo {
    pub x: f64,
    pub y: f64,
    pub ir_size: i64,
}

impl Default for IrSourceInfo {
    fn default() -> Self {
        IrSourceInfo {
            x: 0.0,
            y: 0.0,
            ir_size: 0,
        }
    }
}
