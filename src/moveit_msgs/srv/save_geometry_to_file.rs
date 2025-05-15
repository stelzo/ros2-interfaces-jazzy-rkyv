use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveGeometryToFileRequest {
    pub file_path_and_name: ::std::string::String,
}

impl Default for SaveGeometryToFileRequest {
    fn default() -> Self {
        SaveGeometryToFileRequest {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveGeometryToFileResponse {
    pub success: bool,
}

impl Default for SaveGeometryToFileResponse {
    fn default() -> Self {
        SaveGeometryToFileResponse { success: false }
    }
}

pub struct SaveGeometryToFile;
