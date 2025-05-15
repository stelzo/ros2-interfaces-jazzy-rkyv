use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileReadRequest {
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub size: u64,
}

impl Default for FileReadRequest {
    fn default() -> Self {
        FileReadRequest {
            file_path: ::std::string::String::new(),
            offset: 0,
            size: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileReadResponse {
    pub data: Vec<u8>,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileReadResponse {
    fn default() -> Self {
        FileReadResponse {
            data: Vec::new(),
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileRead;
