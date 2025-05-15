use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HardwareInterface {
    pub name: ::std::string::String,
    pub is_available: bool,
    pub is_claimed: bool,
}

impl Default for HardwareInterface {
    fn default() -> Self {
        HardwareInterface {
            name: ::std::string::String::new(),
            is_available: false,
            is_claimed: false,
        }
    }
}
