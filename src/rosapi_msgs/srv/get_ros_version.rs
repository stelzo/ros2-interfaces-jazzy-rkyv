use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetROSVersionRequest {}

impl Default for GetROSVersionRequest {
    fn default() -> Self {
        GetROSVersionRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetROSVersionResponse {
    pub version: u8,
    pub distro: ::std::string::String,
}

impl Default for GetROSVersionResponse {
    fn default() -> Self {
        GetROSVersionResponse {
            version: 0,
            distro: ::std::string::String::new(),
        }
    }
}

pub struct GetROSVersion;
