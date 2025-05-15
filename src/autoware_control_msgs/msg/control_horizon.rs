use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControlHorizon {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub control_time: crate::builtin_interfaces::msg::Time,
    pub time_step_ms: f32,
    pub controls: Vec<crate::autoware_control_msgs::msg::Control>,
}

impl Default for ControlHorizon {
    fn default() -> Self {
        ControlHorizon {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            control_time: crate::builtin_interfaces::msg::Time::default(),
            time_step_ms: 0.0,
            controls: Vec::new(),
        }
    }
}
