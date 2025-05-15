use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestHeader {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for TestHeader {
    fn default() -> Self {
        TestHeader {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}
