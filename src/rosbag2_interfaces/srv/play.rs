use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PlayRequest {
    pub start_offset: crate::builtin_interfaces::msg::Time,
    pub playback_duration: crate::builtin_interfaces::msg::Duration,
    pub playback_until_timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for PlayRequest {
    fn default() -> Self {
        PlayRequest {
            start_offset: crate::builtin_interfaces::msg::Time::default(),
            playback_duration: crate::builtin_interfaces::msg::Duration::default(),
            playback_until_timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PlayResponse {
    pub success: bool,
}

impl Default for PlayResponse {
    fn default() -> Self {
        PlayResponse { success: false }
    }
}

pub struct Play;
