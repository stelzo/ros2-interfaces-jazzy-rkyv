use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPenRequest {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub width: u8,
    pub off: u8,
}

impl Default for SetPenRequest {
    fn default() -> Self {
        SetPenRequest {
            r: 0,
            g: 0,
            b: 0,
            width: 0,
            off: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPenResponse {}

impl Default for SetPenResponse {
    fn default() -> Self {
        SetPenResponse {}
    }
}

pub struct SetPen;
