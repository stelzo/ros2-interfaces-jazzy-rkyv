use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleQuery {
    pub spacetime: crate::rmf_traffic_msgs::msg::ScheduleQuerySpacetime,
    pub participants: crate::rmf_traffic_msgs::msg::ScheduleQueryParticipants,
}

impl Default for ScheduleQuery {
    fn default() -> Self {
        ScheduleQuery {
            spacetime: crate::rmf_traffic_msgs::msg::ScheduleQuerySpacetime::default(),
            participants: crate::rmf_traffic_msgs::msg::ScheduleQueryParticipants::default(),
        }
    }
}
