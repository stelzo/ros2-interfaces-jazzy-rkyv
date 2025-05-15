use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Station {
    pub task_id: ::std::string::String,
    pub robot_type: ::std::string::String,
    pub place_name: ::std::string::String,
}

impl Default for Station {
    fn default() -> Self {
        Station {
            task_id: ::std::string::String::new(),
            robot_type: ::std::string::String::new(),
            place_name: ::std::string::String::new(),
        }
    }
}
