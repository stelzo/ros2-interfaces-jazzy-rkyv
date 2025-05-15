use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LaneRequest {
    pub fleet_name: ::std::string::String,
    pub open_lanes: Vec<u64>,
    pub close_lanes: Vec<u64>,
}

impl Default for LaneRequest {
    fn default() -> Self {
        LaneRequest {
            fleet_name: ::std::string::String::new(),
            open_lanes: Vec::new(),
            close_lanes: Vec::new(),
        }
    }
}
