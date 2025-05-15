use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrusionData {
    pub data: Vec<crate::sick_safetyscanners2_interfaces::msg::IntrusionDatum>,
}

impl Default for IntrusionData {
    fn default() -> Self {
        IntrusionData { data: Vec::new() }
    }
}
