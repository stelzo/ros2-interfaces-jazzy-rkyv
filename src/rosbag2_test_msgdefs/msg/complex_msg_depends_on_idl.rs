use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ComplexMsgDependsOnIdl {
    pub a: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexMsgDependsOnIdl {
    fn default() -> Self {
        ComplexMsgDependsOnIdl {
            a: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}
