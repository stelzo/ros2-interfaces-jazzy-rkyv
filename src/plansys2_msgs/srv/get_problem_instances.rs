use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemInstancesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemInstancesRequest {
    fn default() -> Self {
        GetProblemInstancesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemInstancesResponse {
    pub success: bool,
    pub instances: Vec<crate::plansys2_msgs::msg::Param>,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemInstancesResponse {
    fn default() -> Self {
        GetProblemInstancesResponse {
            success: false,
            instances: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetProblemInstances;
