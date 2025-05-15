use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModelPropertiesRequest {
    pub model_name: ::std::string::String,
}

impl Default for GetModelPropertiesRequest {
    fn default() -> Self {
        GetModelPropertiesRequest {
            model_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModelPropertiesResponse {
    pub parent_model_name: ::std::string::String,
    pub canonical_body_name: ::std::string::String,
    pub body_names: Vec<::std::string::String>,
    pub geom_names: Vec<::std::string::String>,
    pub joint_names: Vec<::std::string::String>,
    pub child_model_names: Vec<::std::string::String>,
    pub is_static: bool,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetModelPropertiesResponse {
    fn default() -> Self {
        GetModelPropertiesResponse {
            parent_model_name: ::std::string::String::new(),
            canonical_body_name: ::std::string::String::new(),
            body_names: Vec::new(),
            geom_names: Vec::new(),
            joint_names: Vec::new(),
            child_model_names: Vec::new(),
            is_static: false,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct GetModelProperties;
