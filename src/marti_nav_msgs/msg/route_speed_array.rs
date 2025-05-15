use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RouteSpeedArray {
    pub header: crate::std_msgs::msg::Header,
    pub speeds: Vec<crate::marti_nav_msgs::msg::RouteSpeed>,
}

impl Default for RouteSpeedArray {
    fn default() -> Self {
        RouteSpeedArray {
            header: crate::std_msgs::msg::Header::default(),
            speeds: Vec::new(),
        }
    }
}
