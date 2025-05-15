use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadGeometryFromFileRequest {
    pub file_path_and_name: ::std::string::String,
}

impl Default for LoadGeometryFromFileRequest {
    fn default() -> Self {
        LoadGeometryFromFileRequest {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadGeometryFromFileResponse {
    pub success: bool,
}

impl Default for LoadGeometryFromFileResponse {
    fn default() -> Self {
        LoadGeometryFromFileResponse { success: false }
    }
}

pub struct LoadGeometryFromFile;
