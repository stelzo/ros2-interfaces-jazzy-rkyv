use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainActionsRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainActionsRequest {
    fn default() -> Self {
        GetDomainActionsRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainActionsResponse {
    pub success: bool,
    pub actions: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainActionsResponse {
    fn default() -> Self {
        GetDomainActionsResponse {
            success: false,
            actions: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainActions;
