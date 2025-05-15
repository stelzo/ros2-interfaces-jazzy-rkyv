use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct InteractiveMarkerUpdate {
    pub server_id: ::std::string::String,
    pub seq_num: u64,
    pub r#type: u8,
    pub markers: Vec<crate::visualization_msgs::msg::InteractiveMarker>,
    pub poses: Vec<crate::visualization_msgs::msg::InteractiveMarkerPose>,
    pub erases: Vec<::std::string::String>,
}

impl InteractiveMarkerUpdate {
    pub const KEEP_ALIVE: u8 = 0;
    pub const UPDATE: u8 = 1;
}

impl Default for InteractiveMarkerUpdate {
    fn default() -> Self {
        InteractiveMarkerUpdate {
            server_id: ::std::string::String::new(),
            seq_num: 0,
            r#type: 0,
            markers: Vec::new(),
            poses: Vec::new(),
            erases: Vec::new(),
        }
    }
}
