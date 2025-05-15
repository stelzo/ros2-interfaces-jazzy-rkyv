use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NavSatStatus {
    pub status: i8, // default: -2
    pub service: u16,
}

impl NavSatStatus {
    pub const STATUS_UNKNOWN: i8 = -2;
    pub const STATUS_NO_FIX: i8 = -1;
    pub const STATUS_FIX: i8 = 0;
    pub const STATUS_SBAS_FIX: i8 = 1;
    pub const STATUS_GBAS_FIX: i8 = 2;
    pub const SERVICE_UNKNOWN: u16 = 0;
    pub const SERVICE_GPS: u16 = 1;
    pub const SERVICE_GLONASS: u16 = 2;
    pub const SERVICE_COMPASS: u16 = 4;
    pub const SERVICE_GALILEO: u16 = 8;
}

impl Default for NavSatStatus {
    fn default() -> Self {
        NavSatStatus {
            status: -2,
            service: 0,
        }
    }
}
