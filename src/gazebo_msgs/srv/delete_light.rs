use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteLightRequest {
    pub light_name: ::std::string::String,
}

impl Default for DeleteLightRequest {
    fn default() -> Self {
        DeleteLightRequest {
            light_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteLightResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteLightResponse {
    fn default() -> Self {
        DeleteLightResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct DeleteLight;
