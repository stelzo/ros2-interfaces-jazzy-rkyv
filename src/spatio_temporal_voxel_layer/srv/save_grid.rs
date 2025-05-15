use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveGridRequest {
    pub file_name: ::std::string::String,
}

impl Default for SaveGridRequest {
    fn default() -> Self {
        SaveGridRequest {
            file_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveGridResponse {
    pub map_size_bytes: f64,
    pub status: bool,
}

impl Default for SaveGridResponse {
    fn default() -> Self {
        SaveGridResponse {
            map_size_bytes: 0.0,
            status: false,
        }
    }
}

pub struct SaveGrid;
