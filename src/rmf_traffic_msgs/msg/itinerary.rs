use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Itinerary {
    pub routes: Vec<crate::rmf_traffic_msgs::msg::Route>,
}

impl Default for Itinerary {
    fn default() -> Self {
        Itinerary { routes: Vec::new() }
    }
}
