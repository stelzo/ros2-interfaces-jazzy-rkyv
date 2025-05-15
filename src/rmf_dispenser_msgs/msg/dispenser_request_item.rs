use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DispenserRequestItem {
    pub type_guid: ::std::string::String,
    pub quantity: i32,
    pub compartment_name: ::std::string::String,
}

impl Default for DispenserRequestItem {
    fn default() -> Self {
        DispenserRequestItem {
            type_guid: ::std::string::String::new(),
            quantity: 0,
            compartment_name: ::std::string::String::new(),
        }
    }
}
