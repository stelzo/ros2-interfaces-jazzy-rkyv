use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestTimeArray {
    pub times: Vec<crate::builtin_interfaces::msg::Time>,
}

impl Default for TestTimeArray {
    fn default() -> Self {
        TestTimeArray { times: Vec::new() }
    }
}
