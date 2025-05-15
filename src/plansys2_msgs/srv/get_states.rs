use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetStatesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetStatesRequest {
    fn default() -> Self {
        GetStatesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetStatesResponse {
    pub success: bool,
    pub states: Vec<crate::plansys2_msgs::msg::Node>,
    pub error_info: ::std::string::String,
}

impl Default for GetStatesResponse {
    fn default() -> Self {
        GetStatesResponse {
            success: false,
            states: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetStates;
