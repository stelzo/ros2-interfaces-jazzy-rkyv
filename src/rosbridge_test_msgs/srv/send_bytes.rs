use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SendBytesRequest {
    pub count: i64,
}

impl Default for SendBytesRequest {
    fn default() -> Self {
        SendBytesRequest { count: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SendBytesResponse {
    pub data: ::std::string::String,
}

impl Default for SendBytesResponse {
    fn default() -> Self {
        SendBytesResponse {
            data: ::std::string::String::new(),
        }
    }
}

pub struct SendBytes;
