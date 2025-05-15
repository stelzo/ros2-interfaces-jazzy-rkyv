use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EsfRAWBlock {
    pub data: u32,
    pub s_t_tag: u32,
}

impl EsfRAWBlock {
    pub const DATA_FIELD_MASK: u32 = 16777215;
    pub const DATA_TYPE_MASK: u32 = 4278190080;
}

impl Default for EsfRAWBlock {
    fn default() -> Self {
        EsfRAWBlock {
            data: 0,
            s_t_tag: 0,
        }
    }
}
