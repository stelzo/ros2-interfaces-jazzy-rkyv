use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GearCmd {
    pub header: crate::std_msgs::msg::Header,
    pub cmd: crate::ds_dbw_msgs::msg::Gear,
}

impl Default for GearCmd {
    fn default() -> Self {
        GearCmd {
            header: crate::std_msgs::msg::Header::default(),
            cmd: crate::ds_dbw_msgs::msg::Gear::default(),
        }
    }
}
