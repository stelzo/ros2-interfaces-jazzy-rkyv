use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetHardwareComponentStateRequest {
    pub name: ::std::string::String,
    pub target_state: crate::lifecycle_msgs::msg::State,
}

impl Default for SetHardwareComponentStateRequest {
    fn default() -> Self {
        SetHardwareComponentStateRequest {
            name: ::std::string::String::new(),
            target_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetHardwareComponentStateResponse {
    pub ok: bool,
    pub state: crate::lifecycle_msgs::msg::State,
}

impl Default for SetHardwareComponentStateResponse {
    fn default() -> Self {
        SetHardwareComponentStateResponse {
            ok: false,
            state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

pub struct SetHardwareComponentState;
