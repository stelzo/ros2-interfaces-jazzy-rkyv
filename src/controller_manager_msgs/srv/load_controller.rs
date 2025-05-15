use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadControllerRequest {
    pub name: ::std::string::String,
}

impl Default for LoadControllerRequest {
    fn default() -> Self {
        LoadControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadControllerResponse {
    pub ok: bool,
}

impl Default for LoadControllerResponse {
    fn default() -> Self {
        LoadControllerResponse { ok: false }
    }
}

pub struct LoadController;
