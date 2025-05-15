use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorsVelocity {
    pub header: crate::std_msgs::msg::Header,
    pub angular: crate::actuator_msgs::msg::ActuatorsAngularVelocity,
    pub linear: crate::actuator_msgs::msg::ActuatorsLinearVelocity,
}

impl Default for ActuatorsVelocity {
    fn default() -> Self {
        ActuatorsVelocity {
            header: crate::std_msgs::msg::Header::default(),
            angular: crate::actuator_msgs::msg::ActuatorsAngularVelocity::default(),
            linear: crate::actuator_msgs::msg::ActuatorsLinearVelocity::default(),
        }
    }
}
