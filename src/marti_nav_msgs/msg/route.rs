use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Route {
    pub header: crate::std_msgs::msg::Header,
    pub route_points: Vec<crate::marti_nav_msgs::msg::RoutePoint>,
    pub properties: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            header: crate::std_msgs::msg::Header::default(),
            route_points: Vec::new(),
            properties: Vec::new(),
        }
    }
}
