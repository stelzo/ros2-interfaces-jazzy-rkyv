use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EmitterSendStringRequest {
    pub value: ::std::string::String,
}

impl Default for EmitterSendStringRequest {
    fn default() -> Self {
        EmitterSendStringRequest {
            value: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EmitterSendStringResponse {
    pub result: i64,
}

impl Default for EmitterSendStringResponse {
    fn default() -> Self {
        EmitterSendStringResponse { result: 0 }
    }
}

pub struct EmitterSendString;
