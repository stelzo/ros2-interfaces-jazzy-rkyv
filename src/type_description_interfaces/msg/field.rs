use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Field {
    pub name: ::std::string::String,
    pub r#type: crate::type_description_interfaces::msg::FieldType,
    pub default_value: ::std::string::String,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: ::std::string::String::new(),
            r#type: crate::type_description_interfaces::msg::FieldType::default(),
            default_value: ::std::string::String::new(),
        }
    }
}
