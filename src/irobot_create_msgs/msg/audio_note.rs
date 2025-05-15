use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AudioNote {
    pub frequency: u16,
    pub max_runtime: crate::builtin_interfaces::msg::Duration,
}

impl Default for AudioNote {
    fn default() -> Self {
        AudioNote {
            frequency: 0,
            max_runtime: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}
