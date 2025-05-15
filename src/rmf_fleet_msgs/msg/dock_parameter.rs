use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DockParameter {
    pub start: ::std::string::String,
    pub finish: ::std::string::String,
    pub path: Vec<crate::rmf_fleet_msgs::msg::Location>,
}

impl Default for DockParameter {
    fn default() -> Self {
        DockParameter {
            start: ::std::string::String::new(),
            finish: ::std::string::String::new(),
            path: Vec::new(),
        }
    }
}
