use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ESFMeasDataItem {
    pub data_field: u32,
    pub data_type: u8,
}

impl Default for ESFMeasDataItem {
    fn default() -> Self {
        ESFMeasDataItem {
            data_field: 0,
            data_type: 0,
        }
    }
}
