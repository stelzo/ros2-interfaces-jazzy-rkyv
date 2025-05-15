use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotStateEvent {
    pub state: u8,
}

impl RobotStateEvent {
    pub const ONLINE: u8 = 1;
    pub const OFFLINE: u8 = 0;
}

impl Default for RobotStateEvent {
    fn default() -> Self {
        RobotStateEvent { state: 0 }
    }
}
