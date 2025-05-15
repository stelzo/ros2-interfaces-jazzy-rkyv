use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SupervisorHeartbeat {
    pub all_sessions: Vec<crate::rmf_door_msgs::msg::DoorSessions>,
}

impl Default for SupervisorHeartbeat {
    fn default() -> Self {
        SupervisorHeartbeat {
            all_sessions: Vec::new(),
        }
    }
}
