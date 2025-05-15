use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainActionDetailsRequest {
    pub action: ::std::string::String,
    pub parameters: Vec<::std::string::String>,
}

impl Default for GetDomainActionDetailsRequest {
    fn default() -> Self {
        GetDomainActionDetailsRequest {
            action: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainActionDetailsResponse {
    pub action: crate::plansys2_msgs::msg::Action,
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainActionDetailsResponse {
    fn default() -> Self {
        GetDomainActionDetailsResponse {
            action: crate::plansys2_msgs::msg::Action::default(),
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainActionDetails;
