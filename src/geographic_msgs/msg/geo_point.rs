use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl Default for GeoPoint {
    fn default() -> Self {
        GeoPoint {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}
