use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LaneletMapMetaData {
    pub header: crate::std_msgs::msg::Header,
    pub metadata_list: Vec<crate::autoware_map_msgs::msg::LaneletMapCellMetaData>,
}

impl Default for LaneletMapMetaData {
    fn default() -> Self {
        LaneletMapMetaData {
            header: crate::std_msgs::msg::Header::default(),
            metadata_list: Vec::new(),
        }
    }
}
