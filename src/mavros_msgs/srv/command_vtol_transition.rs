use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandVtolTransitionRequest {
    pub header: crate::std_msgs::msg::Header,
    pub state: u8,
}

impl CommandVtolTransitionRequest {
    pub const STATE_MC: u8 = 3;
    pub const STATE_FW: u8 = 4;
}

impl Default for CommandVtolTransitionRequest {
    fn default() -> Self {
        CommandVtolTransitionRequest {
            header: crate::std_msgs::msg::Header::default(),
            state: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandVtolTransitionResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandVtolTransitionResponse {
    fn default() -> Self {
        CommandVtolTransitionResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandVtolTransition;
