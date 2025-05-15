use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ProjectedSonarImage {
    pub header: crate::std_msgs::msg::Header,
    pub ping_info: crate::marine_acoustic_msgs::msg::PingInfo,
    pub beam_directions: Vec<crate::geometry_msgs::msg::Vector3>,
    pub ranges: Vec<f32>,
    pub image: crate::marine_acoustic_msgs::msg::SonarImageData,
}

impl Default for ProjectedSonarImage {
    fn default() -> Self {
        ProjectedSonarImage {
            header: crate::std_msgs::msg::Header::default(),
            ping_info: crate::marine_acoustic_msgs::msg::PingInfo::default(),
            beam_directions: Vec::new(),
            ranges: Vec::new(),
            image: crate::marine_acoustic_msgs::msg::SonarImageData::default(),
        }
    }
}
