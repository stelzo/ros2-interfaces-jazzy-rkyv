use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ShortVariedMultiNested {
    pub short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested,
}

impl Default for ShortVariedMultiNested {
    fn default() -> Self {
        ShortVariedMultiNested {
            short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested::default(),
        }
    }
}
