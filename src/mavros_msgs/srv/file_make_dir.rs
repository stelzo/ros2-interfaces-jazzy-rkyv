use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileMakeDirRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileMakeDirRequest {
    fn default() -> Self {
        FileMakeDirRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileMakeDirResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileMakeDirResponse {
    fn default() -> Self {
        FileMakeDirResponse {
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileMakeDir;
