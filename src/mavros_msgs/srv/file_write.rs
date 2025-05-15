use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileWriteRequest {
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub data: Vec<u8>,
}

impl Default for FileWriteRequest {
    fn default() -> Self {
        FileWriteRequest {
            file_path: ::std::string::String::new(),
            offset: 0,
            data: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileWriteResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileWriteResponse {
    fn default() -> Self {
        FileWriteResponse {
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileWrite;
