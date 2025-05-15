use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetMavFrameRequest {
    pub mav_frame: u8,
}

impl SetMavFrameRequest {
    pub const FRAME_GLOBAL: u8 = 0;
    pub const FRAME_LOCAL_NED: u8 = 1;
    pub const FRAME_MISSION: u8 = 2;
    pub const FRAME_GLOBAL_RELATIVE_ALT: u8 = 3;
    pub const FRAME_LOCAL_ENU: u8 = 4;
    pub const FRAME_GLOBAL_INT: u8 = 5;
    pub const FRAME_GLOBAL_RELATIVE_ALT_INT: u8 = 6;
    pub const FRAME_LOCAL_OFFSET_NED: u8 = 7;
    pub const FRAME_BODY_NED: u8 = 8;
    pub const FRAME_BODY_OFFSET_NED: u8 = 9;
    pub const FRAME_GLOBAL_TERRAIN_ALT: u8 = 10;
    pub const FRAME_GLOBAL_TERRAIN_ALT_INT: u8 = 11;
    pub const FRAME_BODY_FRD: u8 = 12;
    pub const FRAME_RESERVED_13: u8 = 13;
    pub const FRAME_RESERVED_14: u8 = 14;
    pub const FRAME_RESERVED_15: u8 = 15;
    pub const FRAME_RESERVED_16: u8 = 16;
    pub const FRAME_RESERVED_17: u8 = 17;
    pub const FRAME_RESERVED_18: u8 = 18;
    pub const FRAME_RESERVED_19: u8 = 19;
    pub const FRAME_LOCAL_FRD: u8 = 20;
    pub const FRAME_LOCAL_FLU: u8 = 21;
}

impl Default for SetMavFrameRequest {
    fn default() -> Self {
        SetMavFrameRequest { mav_frame: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetMavFrameResponse {
    pub success: bool,
}

impl Default for SetMavFrameResponse {
    fn default() -> Self {
        SetMavFrameResponse { success: false }
    }
}

pub struct SetMavFrame;
