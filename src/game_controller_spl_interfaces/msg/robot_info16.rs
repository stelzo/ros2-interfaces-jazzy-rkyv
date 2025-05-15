use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotInfo16 {
    pub penalty: u8,
    pub secs_till_unpenalised: u8,
}

impl RobotInfo16 {
    pub const PENALTY_NONE: u8 = 0;
    pub const PENALTY_SPL_ILLEGAL_BALL_CONTACT: u8 = 1;
    pub const PENALTY_SPL_PLAYER_PUSHING: u8 = 2;
    pub const PENALTY_SPL_ILLEGAL_MOTION_IN_SET: u8 = 3;
    pub const PENALTY_SPL_INACTIVE_PLAYER: u8 = 4;
    pub const PENALTY_SPL_ILLEGAL_POSITION: u8 = 5;
    pub const PENALTY_SPL_LEAVING_THE_FIELD: u8 = 6;
    pub const PENALTY_SPL_REQUEST_FOR_PICKUP: u8 = 7;
    pub const PENALTY_SPL_LOCAL_GAME_STUCK: u8 = 8;
    pub const PENALTY_SPL_ILLEGAL_POSITION_IN_SET: u8 = 9;
    pub const PENALTY_SPL_PLAYER_STANCE: u8 = 10;
    pub const PENALTY_SPL_ILLEGAL_MOTION_IN_INITIAL: u8 = 11;
    pub const PENALTY_SUBSTITUTE: u8 = 14;
    pub const PENALTY_MANUAL: u8 = 15;
}

impl Default for RobotInfo16 {
    fn default() -> Self {
        RobotInfo16 {
            penalty: 0,
            secs_till_unpenalised: 0,
        }
    }
}
