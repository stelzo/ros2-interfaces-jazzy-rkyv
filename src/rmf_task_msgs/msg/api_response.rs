use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApiResponse {
    pub r#type: u8,
    pub json_msg: ::std::string::String,
    pub request_id: ::std::string::String,
}

impl ApiResponse {
    pub const TYPE_UNINITIALIZED: u8 = 0;
    pub const TYPE_ACKNOWLEDGE: u8 = 1;
    pub const TYPE_RESPONDING: u8 = 2;
}

impl Default for ApiResponse {
    fn default() -> Self {
        ApiResponse {
            r#type: 0,
            json_msg: ::std::string::String::new(),
            request_id: ::std::string::String::new(),
        }
    }
}
