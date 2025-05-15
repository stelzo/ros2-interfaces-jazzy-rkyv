use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileCloseRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileCloseRequest {
    fn default() -> Self {
        FileCloseRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileCloseResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileCloseResponse {
    fn default() -> Self {
        FileCloseResponse {
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileClose;
