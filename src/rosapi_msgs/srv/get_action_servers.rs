use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetActionServersRequest {}

impl Default for GetActionServersRequest {
    fn default() -> Self {
        GetActionServersRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetActionServersResponse {
    pub action_servers: Vec<::std::string::String>,
}

impl Default for GetActionServersResponse {
    fn default() -> Self {
        GetActionServersResponse {
            action_servers: Vec::new(),
        }
    }
}

pub struct GetActionServers;
