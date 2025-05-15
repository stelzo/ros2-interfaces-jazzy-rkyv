use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HardwareComponentState {
    pub name: ::std::string::String,
    pub r#type: ::std::string::String,
    pub class_type: ::std::string::String,
    pub plugin_name: ::std::string::String,
    pub state: crate::lifecycle_msgs::msg::State,
    pub command_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
    pub state_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
}

impl Default for HardwareComponentState {
    fn default() -> Self {
        HardwareComponentState {
            name: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            class_type: ::std::string::String::new(),
            plugin_name: ::std::string::String::new(),
            state: crate::lifecycle_msgs::msg::State::default(),
            command_interfaces: Vec::new(),
            state_interfaces: Vec::new(),
        }
    }
}
