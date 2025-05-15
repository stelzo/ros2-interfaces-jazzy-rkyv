use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MapFeature {
    pub id: crate::unique_identifier_msgs::msg::UUID,
    pub components: Vec<crate::unique_identifier_msgs::msg::UUID>,
    pub props: Vec<crate::geographic_msgs::msg::KeyValue>,
}

impl Default for MapFeature {
    fn default() -> Self {
        MapFeature {
            id: crate::unique_identifier_msgs::msg::UUID::default(),
            components: Vec::new(),
            props: Vec::new(),
        }
    }
}
