use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CostmapFilterInfo {
    pub header: crate::std_msgs::msg::Header,
    pub r#type: u8,
    pub filter_mask_topic: ::std::string::String,
    pub base: f32,
    pub multiplier: f32,
}

impl Default for CostmapFilterInfo {
    fn default() -> Self {
        CostmapFilterInfo {
            header: crate::std_msgs::msg::Header::default(),
            r#type: 0,
            filter_mask_topic: ::std::string::String::new(),
            base: 0.0,
            multiplier: 0.0,
        }
    }
}
