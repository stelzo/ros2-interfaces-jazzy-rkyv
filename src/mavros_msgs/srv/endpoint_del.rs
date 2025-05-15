use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EndpointDelRequest {
    pub id: u32,
    pub url: ::std::string::String,
    pub r#type: u8,
}

impl EndpointDelRequest {
    pub const TYPE_FCU: u8 = 0;
    pub const TYPE_GCS: u8 = 1;
    pub const TYPE_UAS: u8 = 2;
}

impl Default for EndpointDelRequest {
    fn default() -> Self {
        EndpointDelRequest {
            id: 0,
            url: ::std::string::String::new(),
            r#type: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EndpointDelResponse {
    pub successful: bool,
}

impl Default for EndpointDelResponse {
    fn default() -> Self {
        EndpointDelResponse { successful: false }
    }
}

pub struct EndpointDel;
