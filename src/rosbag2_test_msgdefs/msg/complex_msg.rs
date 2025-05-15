use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ComplexMsg {
    pub b: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexMsg {
    fn default() -> Self {
        ComplexMsg {
            b: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}
