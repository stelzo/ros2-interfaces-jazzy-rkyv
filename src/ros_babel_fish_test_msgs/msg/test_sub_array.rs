use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestSubArray {
    pub ints: Vec<i32>,
    pub strings: Vec<::std::string::String>,
    pub times: [crate::builtin_interfaces::msg::Time; 42],
}

impl Default for TestSubArray {
    fn default() -> Self {
        TestSubArray {
            ints: Vec::new(),
            strings: Vec::new(),
            times: core::array::from_fn(|_| crate::builtin_interfaces::msg::Time::default()),
        }
    }
}
