use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRobotSoftwareVersionRequest {}

impl Default for GetRobotSoftwareVersionRequest {
    fn default() -> Self {
        GetRobotSoftwareVersionRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRobotSoftwareVersionResponse {
    pub major: u32,
    pub minor: u32,
    pub bugfix: u32,
    pub build: u32,
}

impl Default for GetRobotSoftwareVersionResponse {
    fn default() -> Self {
        GetRobotSoftwareVersionResponse {
            major: 0,
            minor: 0,
            bugfix: 0,
            build: 0,
        }
    }
}

pub struct GetRobotSoftwareVersion;
