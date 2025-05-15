use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ImuWithMagneticField {
    pub header: crate::std_msgs::msg::Header,
    pub imu: crate::sensor_msgs::msg::Imu,
    pub field: crate::sensor_msgs::msg::MagneticField,
}

impl Default for ImuWithMagneticField {
    fn default() -> Self {
        ImuWithMagneticField {
            header: crate::std_msgs::msg::Header::default(),
            imu: crate::sensor_msgs::msg::Imu::default(),
            field: crate::sensor_msgs::msg::MagneticField::default(),
        }
    }
}
