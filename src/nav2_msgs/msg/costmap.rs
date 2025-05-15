use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Costmap {
    pub header: crate::std_msgs::msg::Header,
    pub metadata: crate::nav2_msgs::msg::CostmapMetaData,
    pub data: Vec<u8>,
}

impl Default for Costmap {
    fn default() -> Self {
        Costmap {
            header: crate::std_msgs::msg::Header::default(),
            metadata: crate::nav2_msgs::msg::CostmapMetaData::default(),
            data: Vec::new(),
        }
    }
}
