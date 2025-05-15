use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ColaMsgSrvRequest {
    pub request: ::std::string::String,
}

impl Default for ColaMsgSrvRequest {
    fn default() -> Self {
        ColaMsgSrvRequest {
            request: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ColaMsgSrvResponse {
    pub response: ::std::string::String,
}

impl Default for ColaMsgSrvResponse {
    fn default() -> Self {
        ColaMsgSrvResponse {
            response: ::std::string::String::new(),
        }
    }
}

pub struct ColaMsgSrv;
