use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct InterfaceVersionRequest {}

impl Default for InterfaceVersionRequest {
    fn default() -> Self {
        InterfaceVersionRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct InterfaceVersionResponse {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Default for InterfaceVersionResponse {
    fn default() -> Self {
        InterfaceVersionResponse {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

pub struct InterfaceVersion;
