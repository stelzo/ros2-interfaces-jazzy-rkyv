use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetAnalogOutputRequest {
    pub data: crate::ur_msgs::msg::Analog,
}

impl Default for SetAnalogOutputRequest {
    fn default() -> Self {
        SetAnalogOutputRequest {
            data: crate::ur_msgs::msg::Analog::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetAnalogOutputResponse {
    pub success: bool,
}

impl Default for SetAnalogOutputResponse {
    fn default() -> Self {
        SetAnalogOutputResponse { success: false }
    }
}

pub struct SetAnalogOutput;
