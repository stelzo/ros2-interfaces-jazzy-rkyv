use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IngestorRequestItem {
    pub type_guid: ::std::string::String,
    pub quantity: i32,
    pub compartment_name: ::std::string::String,
}

impl Default for IngestorRequestItem {
    fn default() -> Self {
        IngestorRequestItem {
            type_guid: ::std::string::String::new(),
            quantity: 0,
            compartment_name: ::std::string::String::new(),
        }
    }
}
