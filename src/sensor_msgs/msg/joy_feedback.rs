use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JoyFeedback {
    pub r#type: u8,
    pub id: u8,
    pub intensity: f32,
}

impl JoyFeedback {
    pub const TYPE_LED: u8 = 0;
    pub const TYPE_RUMBLE: u8 = 1;
    pub const TYPE_BUZZER: u8 = 2;
}

impl Default for JoyFeedback {
    fn default() -> Self {
        JoyFeedback {
            r#type: 0,
            id: 0,
            intensity: 0.0,
        }
    }
}
