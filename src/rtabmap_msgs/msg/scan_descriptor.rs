use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScanDescriptor {
    pub header: crate::std_msgs::msg::Header,
    pub scan: crate::sensor_msgs::msg::LaserScan,
    pub scan_cloud: crate::sensor_msgs::msg::PointCloud2,
    pub global_descriptor: crate::rtabmap_msgs::msg::GlobalDescriptor,
}

impl Default for ScanDescriptor {
    fn default() -> Self {
        ScanDescriptor {
            header: crate::std_msgs::msg::Header::default(),
            scan: crate::sensor_msgs::msg::LaserScan::default(),
            scan_cloud: crate::sensor_msgs::msg::PointCloud2::default(),
            global_descriptor: crate::rtabmap_msgs::msg::GlobalDescriptor::default(),
        }
    }
}
