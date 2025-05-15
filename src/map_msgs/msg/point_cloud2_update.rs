use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PointCloud2Update {
    pub header: crate::std_msgs::msg::Header,
    pub r#type: u32,
    pub points: crate::sensor_msgs::msg::PointCloud2,
}

impl PointCloud2Update {
    pub const ADD: u32 = 0;
    pub const DELETE: u32 = 1;
}

impl Default for PointCloud2Update {
    fn default() -> Self {
        PointCloud2Update {
            header: crate::std_msgs::msg::Header::default(),
            r#type: 0,
            points: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}
