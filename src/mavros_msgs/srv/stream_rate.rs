use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StreamRateRequest {
    pub stream_id: u8,
    pub message_rate: u16,
    pub on_off: bool,
}

impl StreamRateRequest {
    pub const STREAM_ALL: u8 = 0;
    pub const STREAM_RAW_SENSORS: u8 = 1;
    pub const STREAM_EXTENDED_STATUS: u8 = 2;
    pub const STREAM_RC_CHANNELS: u8 = 3;
    pub const STREAM_RAW_CONTROLLER: u8 = 4;
    pub const STREAM_POSITION: u8 = 6;
    pub const STREAM_EXTRA1: u8 = 10;
    pub const STREAM_EXTRA2: u8 = 11;
    pub const STREAM_EXTRA3: u8 = 12;
}

impl Default for StreamRateRequest {
    fn default() -> Self {
        StreamRateRequest {
            stream_id: 0,
            message_rate: 0,
            on_off: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StreamRateResponse {}

impl Default for StreamRateResponse {
    fn default() -> Self {
        StreamRateResponse {}
    }
}

pub struct StreamRate;
