use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AutoFocusCtrl {
    pub auto_focus_mode: u8,
    pub trigger_auto_focus: bool,
}

impl AutoFocusCtrl {
    pub const AF_MODE_AUTO: u8 = 0;
    pub const AF_MODE_MACRO: u8 = 1;
    pub const AF_MODE_CONTINUOUS_VIDEO: u8 = 2;
    pub const AF_MODE_CONTINUOUS_PICTURE: u8 = 3;
    pub const AF_MODE_EDOF: u8 = 4;
}

impl Default for AutoFocusCtrl {
    fn default() -> Self {
        AutoFocusCtrl {
            auto_focus_mode: 0,
            trigger_auto_focus: false,
        }
    }
}
