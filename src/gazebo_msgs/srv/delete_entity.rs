use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteEntityRequest {
    pub name: ::std::string::String,
}

impl Default for DeleteEntityRequest {
    fn default() -> Self {
        DeleteEntityRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteEntityResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteEntityResponse {
    fn default() -> Self {
        DeleteEntityResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct DeleteEntity;
