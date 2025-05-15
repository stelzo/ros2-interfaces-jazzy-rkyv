use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UBXRxmCor {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub ebno: u8,
    pub status_info: crate::ublox_ubx_msgs::msg::CorStatusInfo,
    pub msg_type: u16,
    pub msg_sub_type: u16,
}

impl Default for UBXRxmCor {
    fn default() -> Self {
        UBXRxmCor {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            ebno: 0,
            status_info: crate::ublox_ubx_msgs::msg::CorStatusInfo::default(),
            msg_type: 0,
            msg_sub_type: 0,
        }
    }
}
