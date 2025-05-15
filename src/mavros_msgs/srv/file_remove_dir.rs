use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileRemoveDirRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileRemoveDirRequest {
    fn default() -> Self {
        FileRemoveDirRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileRemoveDirResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveDirResponse {
    fn default() -> Self {
        FileRemoveDirResponse {
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileRemoveDir;
