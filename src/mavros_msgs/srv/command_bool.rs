use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandBoolRequest {
    pub value: bool,
}

impl Default for CommandBoolRequest {
    fn default() -> Self {
        CommandBoolRequest { value: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandBoolResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandBoolResponse {
    fn default() -> Self {
        CommandBoolResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandBool;
