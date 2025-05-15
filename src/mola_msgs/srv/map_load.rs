use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MapLoadRequest {
    pub map_path: ::std::string::String,
}

impl Default for MapLoadRequest {
    fn default() -> Self {
        MapLoadRequest {
            map_path: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MapLoadResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MapLoadResponse {
    fn default() -> Self {
        MapLoadResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

pub struct MapLoad;
