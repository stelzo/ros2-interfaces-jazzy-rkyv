use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct KillRequest {
    pub name: ::std::string::String,
}

impl Default for KillRequest {
    fn default() -> Self {
        KillRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct KillResponse {}

impl Default for KillResponse {
    fn default() -> Self {
        KillResponse {}
    }
}

pub struct Kill;
