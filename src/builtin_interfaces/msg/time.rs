use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Time {
    pub sec: i32,
    pub nanosec: u32,
}

impl Default for Time {
    fn default() -> Self {
        Time { sec: 0, nanosec: 0 }
    }
}
