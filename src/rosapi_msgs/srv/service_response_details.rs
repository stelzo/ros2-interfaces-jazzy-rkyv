use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceResponseDetailsRequest {
    pub r#type: ::std::string::String,
}

impl Default for ServiceResponseDetailsRequest {
    fn default() -> Self {
        ServiceResponseDetailsRequest {
            r#type: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceResponseDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ServiceResponseDetailsResponse {
    fn default() -> Self {
        ServiceResponseDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

pub struct ServiceResponseDetails;
