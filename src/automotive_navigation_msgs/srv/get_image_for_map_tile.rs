use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetImageForMapTileRequest {
    pub tile_id: ::std::string::String,
}

impl Default for GetImageForMapTileRequest {
    fn default() -> Self {
        GetImageForMapTileRequest {
            tile_id: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetImageForMapTileResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetImageForMapTileResponse {
    fn default() -> Self {
        GetImageForMapTileResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct GetImageForMapTile;
