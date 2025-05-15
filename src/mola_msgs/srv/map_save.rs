use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MapSaveRequest {
    pub map_path: ::std::string::String,
}

impl Default for MapSaveRequest {
    fn default() -> Self {
        MapSaveRequest {
            map_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MapSaveResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MapSaveResponse {
    fn default() -> Self {
        MapSaveResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

pub struct MapSave;
