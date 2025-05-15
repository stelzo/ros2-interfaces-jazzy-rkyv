use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileListRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileListRequest {
    fn default() -> Self {
        FileListRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileListResponse {
    pub list: Vec<crate::mavros_msgs::msg::FileEntry>,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileListResponse {
    fn default() -> Self {
        FileListResponse {
            list: Vec::new(),
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileList;
