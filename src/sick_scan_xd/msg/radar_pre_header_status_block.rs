use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RadarPreHeaderStatusBlock {
    pub uitelegramcount: u32,
    pub uicyclecount: u32,
    pub udisystemcountscan: u32,
    pub udisystemcounttransmit: u32,
    pub uiinputs: u16,
    pub uioutputs: u16,
}

impl Default for RadarPreHeaderStatusBlock {
    fn default() -> Self {
        RadarPreHeaderStatusBlock {
            uitelegramcount: 0,
            uicyclecount: 0,
            udisystemcountscan: 0,
            udisystemcounttransmit: 0,
            uiinputs: 0,
            uioutputs: 0,
        }
    }
}
