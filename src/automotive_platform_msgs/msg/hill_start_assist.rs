use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HillStartAssist {
    pub header: crate::std_msgs::msg::Header,
    pub active: bool,
}

impl Default for HillStartAssist {
    fn default() -> Self {
        HillStartAssist {
            header: crate::std_msgs::msg::Header::default(),
            active: false,
        }
    }
}
