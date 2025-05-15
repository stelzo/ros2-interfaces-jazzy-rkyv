use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PolygonMesh {
    pub header: crate::std_msgs::msg::Header,
    pub cloud: crate::sensor_msgs::msg::PointCloud2,
    pub polygons: Vec<crate::pcl_msgs::msg::Vertices>,
}

impl Default for PolygonMesh {
    fn default() -> Self {
        PolygonMesh {
            header: crate::std_msgs::msg::Header::default(),
            cloud: crate::sensor_msgs::msg::PointCloud2::default(),
            polygons: Vec::new(),
        }
    }
}
