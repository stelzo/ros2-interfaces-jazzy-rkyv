use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SbgMagCalib {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for SbgMagCalib {
    fn default() -> Self {
        SbgMagCalib {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}
