use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotConfig {
    pub body_id: ::std::string::String,
    pub body_version: ::std::string::String,
    pub head_id: ::std::string::String,
    pub head_version: ::std::string::String,
}

impl Default for RobotConfig {
    fn default() -> Self {
        RobotConfig {
            body_id: ::std::string::String::new(),
            body_version: ::std::string::String::new(),
            head_id: ::std::string::String::new(),
            head_version: ::std::string::String::new(),
        }
    }
}
