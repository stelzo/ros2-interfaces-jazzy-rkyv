use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestHeaderArray {
    pub header: Vec<crate::std_msgs::msg::Header>,
}

impl Default for TestHeaderArray {
    fn default() -> Self {
        TestHeaderArray { header: Vec::new() }
    }
}
