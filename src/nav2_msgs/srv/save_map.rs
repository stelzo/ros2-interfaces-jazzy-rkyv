use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveMapRequest {
    pub map_topic: ::std::string::String,
    pub map_url: ::std::string::String,
    pub image_format: ::std::string::String,
    pub map_mode: ::std::string::String,
    pub free_thresh: f32,
    pub occupied_thresh: f32,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            map_topic: ::std::string::String::new(),
            map_url: ::std::string::String::new(),
            image_format: ::std::string::String::new(),
            map_mode: ::std::string::String::new(),
            free_thresh: 0.0,
            occupied_thresh: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveMapResponse {
    pub result: bool,
}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse { result: false }
    }
}

pub struct SaveMap;
