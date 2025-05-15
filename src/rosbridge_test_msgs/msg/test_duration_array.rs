use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestDurationArray {
    pub durations: Vec<crate::builtin_interfaces::msg::Duration>,
}

impl Default for TestDurationArray {
    fn default() -> Self {
        TestDurationArray {
            durations: Vec::new(),
        }
    }
}
