use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetUTMZoneRequest {
    pub utm_zone: ::std::string::String,
}

impl Default for SetUTMZoneRequest {
    fn default() -> Self {
        SetUTMZoneRequest {
            utm_zone: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetUTMZoneResponse {}

impl Default for SetUTMZoneResponse {
    fn default() -> Self {
        SetUTMZoneResponse {}
    }
}

pub struct SetUTMZone;
