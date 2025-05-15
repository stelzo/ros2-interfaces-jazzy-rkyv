use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Trackstat {
    pub header: crate::std_msgs::msg::Header,
    pub solution_status: ::std::string::String,
    pub position_type: ::std::string::String,
    pub cutoff: f32,
    pub channels: Vec<crate::novatel_gps_msgs::msg::TrackstatChannel>,
}

impl Default for Trackstat {
    fn default() -> Self {
        Trackstat {
            header: crate::std_msgs::msg::Header::default(),
            solution_status: ::std::string::String::new(),
            position_type: ::std::string::String::new(),
            cutoff: 0.0,
            channels: Vec::new(),
        }
    }
}
