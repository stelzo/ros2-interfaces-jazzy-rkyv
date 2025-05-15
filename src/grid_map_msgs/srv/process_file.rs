use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ProcessFileRequest {
    pub file_path: ::std::string::String,
    pub topic_name: ::std::string::String,
}

impl Default for ProcessFileRequest {
    fn default() -> Self {
        ProcessFileRequest {
            file_path: ::std::string::String::new(),
            topic_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ProcessFileResponse {
    pub success: bool,
}

impl Default for ProcessFileResponse {
    fn default() -> Self {
        ProcessFileResponse { success: false }
    }
}

pub struct ProcessFile;
