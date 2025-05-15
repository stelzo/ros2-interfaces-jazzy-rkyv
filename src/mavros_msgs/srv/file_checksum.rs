use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileChecksumRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileChecksumRequest {
    fn default() -> Self {
        FileChecksumRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileChecksumResponse {
    pub crc32: u32,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileChecksumResponse {
    fn default() -> Self {
        FileChecksumResponse {
            crc32: 0,
            success: false,
            r_errno: 0,
        }
    }
}

pub struct FileChecksum;
