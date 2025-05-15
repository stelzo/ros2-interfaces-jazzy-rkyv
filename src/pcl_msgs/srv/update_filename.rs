use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdateFilenameRequest {
    pub filename: ::std::string::String,
}

impl Default for UpdateFilenameRequest {
    fn default() -> Self {
        UpdateFilenameRequest {
            filename: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdateFilenameResponse {
    pub success: bool,
}

impl Default for UpdateFilenameResponse {
    fn default() -> Self {
        UpdateFilenameResponse { success: false }
    }
}

pub struct UpdateFilename;
