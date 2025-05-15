use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UnloadControllerRequest {
    pub name: ::std::string::String,
}

impl Default for UnloadControllerRequest {
    fn default() -> Self {
        UnloadControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UnloadControllerResponse {
    pub ok: bool,
}

impl Default for UnloadControllerResponse {
    fn default() -> Self {
        UnloadControllerResponse { ok: false }
    }
}

pub struct UnloadController;
