use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SoftwareVersionReq {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
}

impl Default for SoftwareVersionReq {
    fn default() -> Self {
        SoftwareVersionReq {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
        }
    }
}
