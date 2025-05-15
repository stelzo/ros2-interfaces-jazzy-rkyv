use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetWorldPropertiesRequest {}

impl Default for GetWorldPropertiesRequest {
    fn default() -> Self {
        GetWorldPropertiesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetWorldPropertiesResponse {
    pub sim_time: f64,
    pub model_names: Vec<::std::string::String>,
    pub rendering_enabled: bool,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetWorldPropertiesResponse {
    fn default() -> Self {
        GetWorldPropertiesResponse {
            sim_time: 0.0,
            model_names: Vec::new(),
            rendering_enabled: false,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct GetWorldProperties;
