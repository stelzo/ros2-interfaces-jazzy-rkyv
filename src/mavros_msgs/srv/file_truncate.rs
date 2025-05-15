use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileTruncateRequest {
    pub file_path: ::std::string::String,
    pub length: u64,
}

impl Default for FileTruncateRequest {
    fn default() -> Self {
        FileTruncateRequest {
            file_path: ::std::string::String::new(),
            length: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileTruncateResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileTruncateResponse {
    fn default() -> Self {
        FileTruncateResponse {
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileTruncate;
