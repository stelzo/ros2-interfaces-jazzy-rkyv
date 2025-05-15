use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetUserdataRequest {
    pub userdata_key: ::std::string::String,
}

impl Default for GetUserdataRequest {
    fn default() -> Self {
        GetUserdataRequest {
            userdata_key: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetUserdataResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub userdata: Vec<crate::flexbe_msgs::msg::UserdataInfo>,
}

impl Default for GetUserdataResponse {
    fn default() -> Self {
        GetUserdataResponse {
            success: false,
            message: ::std::string::String::new(),
            userdata: Vec::new(),
        }
    }
}

pub struct GetUserdata;
