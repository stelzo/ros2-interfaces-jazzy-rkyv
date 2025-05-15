use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VisionClass {
    pub class_id: u16,
    pub class_name: ::std::string::String,
}

impl Default for VisionClass {
    fn default() -> Self {
        VisionClass {
            class_id: 0,
            class_name: ::std::string::String::new(),
        }
    }
}
