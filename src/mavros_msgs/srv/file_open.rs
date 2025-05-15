use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileOpenRequest {
    pub file_path: ::std::string::String,
    pub mode: u8,
}

impl FileOpenRequest {
    pub const MODE_READ: u8 = 0;
    pub const MODE_WRITE: u8 = 1;
    pub const MODE_CREATE: u8 = 2;
}

impl Default for FileOpenRequest {
    fn default() -> Self {
        FileOpenRequest {
            file_path: ::std::string::String::new(),
            mode: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileOpenResponse {
    pub size: u32,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileOpenResponse {
    fn default() -> Self {
        FileOpenResponse {
            size: 0,
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileOpen;
