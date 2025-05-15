use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MapMatching {
    pub status: u8,
}

impl MapMatching {
    pub const MAP_MATCHING_NONE: u8 = 0;
    pub const MAP_MATCHING_VALID_NOT_USED: u8 = 1;
    pub const MAP_MATCHING_VALID_AND_USED: u8 = 2;
    pub const MAP_MATCHING_VALID_DEAD_RECKONING: u8 = 3;
}

impl Default for MapMatching {
    fn default() -> Self {
        MapMatching { status: 0 }
    }
}
