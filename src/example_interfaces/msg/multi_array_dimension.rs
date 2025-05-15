use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MultiArrayDimension {
    pub label: ::std::string::String,
    pub size: u32,
    pub stride: u32,
}

impl Default for MultiArrayDimension {
    fn default() -> Self {
        MultiArrayDimension {
            label: ::std::string::String::new(),
            size: 0,
            stride: 0,
        }
    }
}
