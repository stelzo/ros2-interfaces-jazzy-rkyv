use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EndpointAddRequest {
    pub url: ::std::string::String,
    pub r#type: u8,
}

impl EndpointAddRequest {
    pub const TYPE_FCU: u8 = 0;
    pub const TYPE_GCS: u8 = 1;
    pub const TYPE_UAS: u8 = 2;
}

impl Default for EndpointAddRequest {
    fn default() -> Self {
        EndpointAddRequest {
            url: ::std::string::String::new(),
            r#type: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EndpointAddResponse {
    pub successful: bool,
    pub reason: ::std::string::String,
    pub id: u32,
}

impl Default for EndpointAddResponse {
    fn default() -> Self {
        EndpointAddResponse {
            successful: false,
            reason: ::std::string::String::new(),
            id: 0,
        }
    }
}

pub struct EndpointAdd;
