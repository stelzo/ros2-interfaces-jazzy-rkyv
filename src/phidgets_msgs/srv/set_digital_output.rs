use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetDigitalOutputRequest {
    pub index: u16,
    pub state: bool,
}

impl Default for SetDigitalOutputRequest {
    fn default() -> Self {
        SetDigitalOutputRequest {
            index: 0,
            state: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetDigitalOutputResponse {
    pub success: bool,
}

impl Default for SetDigitalOutputResponse {
    fn default() -> Self {
        SetDigitalOutputResponse { success: false }
    }
}

pub struct SetDigitalOutput;
