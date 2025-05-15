use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConfigureControllerRequest {
    pub name: ::std::string::String,
}

impl Default for ConfigureControllerRequest {
    fn default() -> Self {
        ConfigureControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConfigureControllerResponse {
    pub ok: bool,
}

impl Default for ConfigureControllerResponse {
    fn default() -> Self {
        ConfigureControllerResponse { ok: false }
    }
}

pub struct ConfigureController;
