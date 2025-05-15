use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChargingAssignments {
    pub fleet_name: ::std::string::String,
    pub assignments: Vec<crate::rmf_fleet_msgs::msg::ChargingAssignment>,
}

impl Default for ChargingAssignments {
    fn default() -> Self {
        ChargingAssignments {
            fleet_name: ::std::string::String::new(),
            assignments: Vec::new(),
        }
    }
}
