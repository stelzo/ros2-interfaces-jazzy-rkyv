use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VelodynePacket {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: [u8; 1206],
}

impl Default for VelodynePacket {
    fn default() -> Self {
        VelodynePacket {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: [0; 1206],
        }
    }
}
