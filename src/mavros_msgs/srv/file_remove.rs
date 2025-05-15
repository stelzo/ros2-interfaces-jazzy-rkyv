use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileRemoveRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileRemoveRequest {
    fn default() -> Self {
        FileRemoveRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileRemoveResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveResponse {
    fn default() -> Self {
        FileRemoveResponse {
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileRemove;
