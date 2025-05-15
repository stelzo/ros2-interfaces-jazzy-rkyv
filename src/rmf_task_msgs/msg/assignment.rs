use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Assignment {
    pub is_assigned: bool,
    pub fleet_name: ::std::string::String,
    pub expected_robot_name: ::std::string::String,
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment {
            is_assigned: false,
            fleet_name: ::std::string::String::new(),
            expected_robot_name: ::std::string::String::new(),
        }
    }
}
