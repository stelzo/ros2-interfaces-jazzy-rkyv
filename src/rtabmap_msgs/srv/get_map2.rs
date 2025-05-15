use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMap2Request {
    pub global_map: bool,
    pub optimized: bool,
    pub with_images: bool,
    pub with_scans: bool,
    pub with_user_data: bool,
    pub with_grids: bool,
    pub with_words: bool,
    pub with_global_descriptors: bool,
}

impl Default for GetMap2Request {
    fn default() -> Self {
        GetMap2Request {
            global_map: false,
            optimized: false,
            with_images: false,
            with_scans: false,
            with_user_data: false,
            with_grids: false,
            with_words: false,
            with_global_descriptors: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMap2Response {
    pub data: crate::rtabmap_msgs::msg::MapData,
}

impl Default for GetMap2Response {
    fn default() -> Self {
        GetMap2Response {
            data: crate::rtabmap_msgs::msg::MapData::default(),
        }
    }
}

pub struct GetMap2;
