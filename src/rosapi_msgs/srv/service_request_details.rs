use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceRequestDetailsRequest {
    pub r#type: ::std::string::String,
}

impl Default for ServiceRequestDetailsRequest {
    fn default() -> Self {
        ServiceRequestDetailsRequest {
            r#type: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceRequestDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ServiceRequestDetailsResponse {
    fn default() -> Self {
        ServiceRequestDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

pub struct ServiceRequestDetails;
