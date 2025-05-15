use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PointOfInterestArray {
    pub header: crate::std_msgs::msg::Header,
    pub update_num: u16,
    pub point_list: Vec<crate::automotive_navigation_msgs::msg::PointOfInterest>,
}

impl Default for PointOfInterestArray {
    fn default() -> Self {
        PointOfInterestArray {
            header: crate::std_msgs::msg::Header::default(),
            update_num: 0,
            point_list: Vec::new(),
        }
    }
}
