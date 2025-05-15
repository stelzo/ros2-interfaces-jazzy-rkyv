use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorUniqueIdReq {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
}

impl Default for ActuatorUniqueIdReq {
    fn default() -> Self {
        ActuatorUniqueIdReq {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
        }
    }
}
