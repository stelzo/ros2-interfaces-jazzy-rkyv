use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Clean {
    pub start_waypoint: ::std::string::String,
}

impl Default for Clean {
    fn default() -> Self {
        Clean {
            start_waypoint: ::std::string::String::new(),
        }
    }
}
