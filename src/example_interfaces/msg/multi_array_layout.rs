use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MultiArrayLayout {
    pub dim: Vec<crate::example_interfaces::msg::MultiArrayDimension>,
    pub data_offset: u32,
}

impl Default for MultiArrayLayout {
    fn default() -> Self {
        MultiArrayLayout {
            dim: Vec::new(),
            data_offset: 0,
        }
    }
}
