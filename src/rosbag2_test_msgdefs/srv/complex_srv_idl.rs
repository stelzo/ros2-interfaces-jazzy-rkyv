use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ComplexSrvIdlRequest {
    pub req: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexSrvIdlRequest {
    fn default() -> Self {
        ComplexSrvIdlRequest {
            req: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ComplexSrvIdlResponse {
    pub resp: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexSrvIdlResponse {
    fn default() -> Self {
        ComplexSrvIdlResponse {
            resp: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

pub struct ComplexSrvIdl;
