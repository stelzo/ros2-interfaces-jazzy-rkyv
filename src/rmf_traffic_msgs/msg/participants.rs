use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Participants {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub participants: Vec<crate::rmf_traffic_msgs::msg::Participant>,
}

impl Default for Participants {
    fn default() -> Self {
        Participants {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            participants: Vec::new(),
        }
    }
}
