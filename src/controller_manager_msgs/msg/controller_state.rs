use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControllerState {
    pub name: ::std::string::String,
    pub state: ::std::string::String,
    pub r#type: ::std::string::String,
    pub claimed_interfaces: Vec<::std::string::String>,
    pub required_command_interfaces: Vec<::std::string::String>,
    pub required_state_interfaces: Vec<::std::string::String>,
    pub is_chainable: bool,
    pub is_chained: bool,
    pub exported_state_interfaces: Vec<::std::string::String>,
    pub reference_interfaces: Vec<::std::string::String>,
    pub chain_connections: Vec<crate::controller_manager_msgs::msg::ChainConnection>,
}

impl Default for ControllerState {
    fn default() -> Self {
        ControllerState {
            name: ::std::string::String::new(),
            state: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            claimed_interfaces: Vec::new(),
            required_command_interfaces: Vec::new(),
            required_state_interfaces: Vec::new(),
            is_chainable: false,
            is_chained: false,
            exported_state_interfaces: Vec::new(),
            reference_interfaces: Vec::new(),
            chain_connections: Vec::new(),
        }
    }
}
