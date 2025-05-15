use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BasicSrvRequest {
    pub req: ::std::string::String,
}

impl Default for BasicSrvRequest {
    fn default() -> Self {
        BasicSrvRequest {
            req: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BasicSrvResponse {
    pub resp: ::std::string::String,
}

impl Default for BasicSrvResponse {
    fn default() -> Self {
        BasicSrvResponse {
            resp: ::std::string::String::new(),
        }
    }
}

pub struct BasicSrv;
