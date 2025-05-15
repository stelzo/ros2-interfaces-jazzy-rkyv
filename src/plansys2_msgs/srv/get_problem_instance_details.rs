use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemInstanceDetailsRequest {
    pub instance: ::std::string::String,
}

impl Default for GetProblemInstanceDetailsRequest {
    fn default() -> Self {
        GetProblemInstanceDetailsRequest {
            instance: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemInstanceDetailsResponse {
    pub success: bool,
    pub instance: crate::plansys2_msgs::msg::Param,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemInstanceDetailsResponse {
    fn default() -> Self {
        GetProblemInstanceDetailsResponse {
            success: false,
            instance: crate::plansys2_msgs::msg::Param::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetProblemInstanceDetails;
