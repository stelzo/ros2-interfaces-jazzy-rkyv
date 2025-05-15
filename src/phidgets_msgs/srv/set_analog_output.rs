use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetAnalogOutputRequest {
    pub index: u16,
    pub voltage: f64,
}

impl Default for SetAnalogOutputRequest {
    fn default() -> Self {
        SetAnalogOutputRequest {
            index: 0,
            voltage: 0.0,
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
