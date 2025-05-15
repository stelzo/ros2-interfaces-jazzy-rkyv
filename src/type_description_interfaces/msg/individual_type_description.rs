use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IndividualTypeDescription {
    pub type_name: ::std::string::String,
    pub fields: Vec<crate::type_description_interfaces::msg::Field>,
}

impl Default for IndividualTypeDescription {
    fn default() -> Self {
        IndividualTypeDescription {
            type_name: ::std::string::String::new(),
            fields: Vec::new(),
        }
    }
}
