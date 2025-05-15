use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GeographicMapChanges {
    pub header: crate::std_msgs::msg::Header,
    pub diffs: crate::geographic_msgs::msg::GeographicMap,
    pub deletes: Vec<crate::unique_identifier_msgs::msg::UUID>,
}

impl Default for GeographicMapChanges {
    fn default() -> Self {
        GeographicMapChanges {
            header: crate::std_msgs::msg::Header::default(),
            diffs: crate::geographic_msgs::msg::GeographicMap::default(),
            deletes: Vec::new(),
        }
    }
}
