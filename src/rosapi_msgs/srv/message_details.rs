use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MessageDetailsRequest {
    pub r#type: ::std::string::String,
}

impl Default for MessageDetailsRequest {
    fn default() -> Self {
        MessageDetailsRequest {
            r#type: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MessageDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for MessageDetailsResponse {
    fn default() -> Self {
        MessageDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

pub struct MessageDetails;
