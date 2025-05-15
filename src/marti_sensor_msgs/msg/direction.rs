use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Direction {
    pub header: crate::std_msgs::msg::Header,
    pub direction: i8,
}

impl Direction {
    pub const BACKWARD: i8 = -1;
    pub const ZERO: i8 = 0;
    pub const FORWARD: i8 = 1;
    pub const UNKNOWN: i8 = 127;
}

impl Default for Direction {
    fn default() -> Self {
        Direction {
            header: crate::std_msgs::msg::Header::default(),
            direction: 0,
        }
    }
}
