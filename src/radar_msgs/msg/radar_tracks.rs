use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RadarTracks {
    pub header: crate::std_msgs::msg::Header,
    pub tracks: Vec<crate::radar_msgs::msg::RadarTrack>,
}

impl Default for RadarTracks {
    fn default() -> Self {
        RadarTracks {
            header: crate::std_msgs::msg::Header::default(),
            tracks: Vec::new(),
        }
    }
}
