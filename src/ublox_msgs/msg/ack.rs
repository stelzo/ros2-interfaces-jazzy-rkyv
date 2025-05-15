use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Ack {
    pub cls_id: u8,
    pub msg_id: u8,
}

impl Ack {
    pub const CLASS_ID: u8 = 5;
    pub const NACK_MESSAGE_ID: u8 = 0;
    pub const ACK_MESSAGE_ID: u8 = 1;
}

impl Default for Ack {
    fn default() -> Self {
        Ack {
            cls_id: 0,
            msg_id: 0,
        }
    }
}
