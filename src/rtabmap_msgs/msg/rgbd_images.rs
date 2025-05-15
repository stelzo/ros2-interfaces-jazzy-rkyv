use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RGBDImages {
    pub header: crate::std_msgs::msg::Header,
    pub rgbd_images: Vec<crate::rtabmap_msgs::msg::RGBDImage>,
}

impl Default for RGBDImages {
    fn default() -> Self {
        RGBDImages {
            header: crate::std_msgs::msg::Header::default(),
            rgbd_images: Vec::new(),
        }
    }
}
