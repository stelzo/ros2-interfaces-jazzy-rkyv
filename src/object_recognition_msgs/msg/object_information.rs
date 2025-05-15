use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectInformation {
    pub name: ::std::string::String,
    pub ground_truth_mesh: crate::shape_msgs::msg::Mesh,
    pub ground_truth_point_cloud: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for ObjectInformation {
    fn default() -> Self {
        ObjectInformation {
            name: ::std::string::String::new(),
            ground_truth_mesh: crate::shape_msgs::msg::Mesh::default(),
            ground_truth_point_cloud: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}
