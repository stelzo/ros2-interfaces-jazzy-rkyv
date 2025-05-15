use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NovatelFRESETRequest {
    pub target: ::std::string::String,
}

impl Default for NovatelFRESETRequest {
    fn default() -> Self {
        NovatelFRESETRequest {
            target: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NovatelFRESETResponse {
    pub success: bool,
}

impl Default for NovatelFRESETResponse {
    fn default() -> Self {
        NovatelFRESETResponse { success: false }
    }
}

pub struct NovatelFRESET;
