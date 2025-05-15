use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ShortVariedNested {
    pub short_varied: crate::ros2cli_test_interfaces::msg::ShortVaried,
}

impl Default for ShortVariedNested {
    fn default() -> Self {
        ShortVariedNested {
            short_varied: crate::ros2cli_test_interfaces::msg::ShortVaried::default(),
        }
    }
}
