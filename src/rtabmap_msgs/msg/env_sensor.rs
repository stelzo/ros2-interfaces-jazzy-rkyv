use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EnvSensor {
    pub header: crate::std_msgs::msg::Header,
    pub r#type: i32,
    pub value: f64,
}

impl Default for EnvSensor {
    fn default() -> Self {
        EnvSensor {
            header: crate::std_msgs::msg::Header::default(),
            r#type: 0,
            value: 0.0,
        }
    }
}
