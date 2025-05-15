use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestHeaderTwo {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for TestHeaderTwo {
    fn default() -> Self {
        TestHeaderTwo {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}
