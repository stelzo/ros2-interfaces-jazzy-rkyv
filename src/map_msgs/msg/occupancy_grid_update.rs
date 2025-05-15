use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OccupancyGridUpdate {
    pub header: crate::std_msgs::msg::Header,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<i8>,
}

impl Default for OccupancyGridUpdate {
    fn default() -> Self {
        OccupancyGridUpdate {
            header: crate::std_msgs::msg::Header::default(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}
