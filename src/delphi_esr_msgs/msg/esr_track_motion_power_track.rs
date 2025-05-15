use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EsrTrackMotionPowerTrack {
    pub id: u8,
    pub movable_fast: bool,
    pub movable_slow: bool,
    pub moving: bool,
    pub power: i8,
}

impl Default for EsrTrackMotionPowerTrack {
    fn default() -> Self {
        EsrTrackMotionPowerTrack {
            id: 0,
            movable_fast: false,
            movable_slow: false,
            moving: false,
            power: 0,
        }
    }
}
