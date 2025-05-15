use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Heartbeat {}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat {}
    }
}
