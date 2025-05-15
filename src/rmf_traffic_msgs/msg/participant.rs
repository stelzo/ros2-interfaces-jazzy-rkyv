use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Participant {
    pub id: u64,
    pub description: crate::rmf_traffic_msgs::msg::ParticipantDescription,
}

impl Default for Participant {
    fn default() -> Self {
        Participant {
            id: 0,
            description: crate::rmf_traffic_msgs::msg::ParticipantDescription::default(),
        }
    }
}
