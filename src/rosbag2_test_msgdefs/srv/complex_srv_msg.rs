use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ComplexSrvMsgRequest {
    pub req: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexSrvMsgRequest {
    fn default() -> Self {
        ComplexSrvMsgRequest {
            req: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ComplexSrvMsgResponse {
    pub resp: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexSrvMsgResponse {
    fn default() -> Self {
        ComplexSrvMsgResponse {
            resp: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}

pub struct ComplexSrvMsg;
