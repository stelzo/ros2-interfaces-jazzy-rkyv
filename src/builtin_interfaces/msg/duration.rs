use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Duration {
    pub sec: i32,
    pub nanosec: u32,
}

impl Default for Duration {
    fn default() -> Self {
        Duration { sec: 0, nanosec: 0 }
    }
}
