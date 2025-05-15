use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteModelRequest {
    pub model_name: ::std::string::String,
}

impl Default for DeleteModelRequest {
    fn default() -> Self {
        DeleteModelRequest {
            model_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteModelResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteModelResponse {
    fn default() -> Self {
        DeleteModelResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct DeleteModel;
