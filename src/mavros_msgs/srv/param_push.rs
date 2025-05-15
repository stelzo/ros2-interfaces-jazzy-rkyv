use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamPushRequest {}

impl Default for ParamPushRequest {
    fn default() -> Self {
        ParamPushRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamPushResponse {
    pub success: bool,
    pub param_transfered: u32,
}

impl Default for ParamPushResponse {
    fn default() -> Self {
        ParamPushResponse {
            success: false,
            param_transfered: 0,
        }
    }
}

pub struct ParamPush;
