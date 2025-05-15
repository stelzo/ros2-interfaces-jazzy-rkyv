use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Parameter {
    pub name: ::std::string::String,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            name: ::std::string::String::new(),
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}
