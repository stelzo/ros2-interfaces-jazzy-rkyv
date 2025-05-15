use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LaneStates {
    pub fleet_name: ::std::string::String,
    pub closed_lanes: Vec<u64>,
    pub speed_limits: Vec<crate::rmf_fleet_msgs::msg::SpeedLimitedLane>,
}

impl Default for LaneStates {
    fn default() -> Self {
        LaneStates {
            fleet_name: ::std::string::String::new(),
            closed_lanes: Vec::new(),
            speed_limits: Vec::new(),
        }
    }
}
