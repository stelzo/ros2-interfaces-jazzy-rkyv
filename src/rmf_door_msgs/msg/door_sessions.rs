use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DoorSessions {
    pub door_name: ::std::string::String,
    pub sessions: Vec<crate::rmf_door_msgs::msg::Session>,
}

impl Default for DoorSessions {
    fn default() -> Self {
        DoorSessions {
            door_name: ::std::string::String::new(),
            sessions: Vec::new(),
        }
    }
}
