use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamPullRequest {
    pub force_pull: bool,
}

impl Default for ParamPullRequest {
    fn default() -> Self {
        ParamPullRequest { force_pull: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamPullResponse {
    pub success: bool,
    pub param_received: u32,
}

impl Default for ParamPullResponse {
    fn default() -> Self {
        ParamPullResponse {
            success: false,
            param_received: 0,
        }
    }
}

pub struct ParamPull;
