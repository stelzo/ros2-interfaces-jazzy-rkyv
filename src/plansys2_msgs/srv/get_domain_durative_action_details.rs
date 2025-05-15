use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainDurativeActionDetailsRequest {
    pub durative_action: ::std::string::String,
    pub parameters: Vec<::std::string::String>,
}

impl Default for GetDomainDurativeActionDetailsRequest {
    fn default() -> Self {
        GetDomainDurativeActionDetailsRequest {
            durative_action: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainDurativeActionDetailsResponse {
    pub success: bool,
    pub durative_action: crate::plansys2_msgs::msg::DurativeAction,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainDurativeActionDetailsResponse {
    fn default() -> Self {
        GetDomainDurativeActionDetailsResponse {
            success: false,
            durative_action: crate::plansys2_msgs::msg::DurativeAction::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainDurativeActionDetails;
