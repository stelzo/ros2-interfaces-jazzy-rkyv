use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CellularStatus {
    pub status: u8,
    pub failure_reason: u8,
    pub r#type: u8,
    pub quality: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
}

impl Default for CellularStatus {
    fn default() -> Self {
        CellularStatus {
            status: 0,
            failure_reason: 0,
            r#type: 0,
            quality: 0,
            mcc: 0,
            mnc: 0,
            lac: 0,
        }
    }
}
