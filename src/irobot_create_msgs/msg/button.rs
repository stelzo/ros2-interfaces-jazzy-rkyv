use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Button {
    pub header: crate::std_msgs::msg::Header,
    pub is_pressed: bool,
    pub last_start_pressed_time: crate::builtin_interfaces::msg::Time,
    pub last_pressed_duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            header: crate::std_msgs::msg::Header::default(),
            is_pressed: false,
            last_start_pressed_time: crate::builtin_interfaces::msg::Time::default(),
            last_pressed_duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}
