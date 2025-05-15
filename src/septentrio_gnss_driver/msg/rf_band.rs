use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RFBand {
    pub frequency: u32,
    pub bandwidth: u16,
    pub info: u8,
}

impl Default for RFBand {
    fn default() -> Self {
        RFBand {
            frequency: 0,
            bandwidth: 0,
            info: 0,
        }
    }
}
