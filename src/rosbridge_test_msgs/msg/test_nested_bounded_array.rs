use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestNestedBoundedArray {
    pub data: crate::rosbridge_test_msgs::msg::TestFloat32BoundedArray,
}

impl Default for TestNestedBoundedArray {
    fn default() -> Self {
        TestNestedBoundedArray {
            data: crate::rosbridge_test_msgs::msg::TestFloat32BoundedArray::default(),
        }
    }
}
