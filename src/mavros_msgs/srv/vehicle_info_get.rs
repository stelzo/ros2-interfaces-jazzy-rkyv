use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VehicleInfoGetRequest {
    pub sysid: u8,
    pub compid: u8,
    pub get_all: bool,
}

impl VehicleInfoGetRequest {
    pub const GET_MY_SYSID: u8 = 0;
    pub const GET_MY_COMPID: u8 = 0;
}

impl Default for VehicleInfoGetRequest {
    fn default() -> Self {
        VehicleInfoGetRequest {
            sysid: 0,
            compid: 0,
            get_all: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VehicleInfoGetResponse {
    pub success: bool,
    pub vehicles: Vec<crate::mavros_msgs::msg::VehicleInfo>,
}

impl Default for VehicleInfoGetResponse {
    fn default() -> Self {
        VehicleInfoGetResponse {
            success: false,
            vehicles: Vec::new(),
        }
    }
}

pub struct VehicleInfoGet;
