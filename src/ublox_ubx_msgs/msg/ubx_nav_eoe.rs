use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UBXNavEOE {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
}

impl Default for UBXNavEOE {
    fn default() -> Self {
        UBXNavEOE {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
        }
    }
}
