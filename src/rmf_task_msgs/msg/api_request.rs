use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApiRequest {
    pub json_msg: ::std::string::String,
    pub request_id: ::std::string::String,
}

impl Default for ApiRequest {
    fn default() -> Self {
        ApiRequest {
            json_msg: ::std::string::String::new(),
            request_id: ::std::string::String::new(),
        }
    }
}
