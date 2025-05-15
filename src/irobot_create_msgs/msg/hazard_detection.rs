use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HazardDetection {
    pub header: crate::std_msgs::msg::Header,
    pub r#type: u8,
}

impl HazardDetection {
    pub const BACKUP_LIMIT: u8 = 0;
    pub const BUMP: u8 = 1;
    pub const CLIFF: u8 = 2;
    pub const STALL: u8 = 3;
    pub const WHEEL_DROP: u8 = 4;
    pub const OBJECT_PROXIMITY: u8 = 5;
}

impl Default for HazardDetection {
    fn default() -> Self {
        HazardDetection {
            header: crate::std_msgs::msg::Header::default(),
            r#type: 0,
        }
    }
}
