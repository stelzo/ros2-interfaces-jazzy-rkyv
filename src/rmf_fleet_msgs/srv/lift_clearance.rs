use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LiftClearanceRequest {
    pub robot_name: ::std::string::String,
    pub lift_name: ::std::string::String,
}

impl Default for LiftClearanceRequest {
    fn default() -> Self {
        LiftClearanceRequest {
            robot_name: ::std::string::String::new(),
            lift_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LiftClearanceResponse {
    pub decision: u32,
}

impl LiftClearanceResponse {
    pub const DECISION_CLEAR: u32 = 1;
    pub const DECISION_CROWDED: u32 = 2;
}

impl Default for LiftClearanceResponse {
    fn default() -> Self {
        LiftClearanceResponse { decision: 0 }
    }
}

pub struct LiftClearance;
