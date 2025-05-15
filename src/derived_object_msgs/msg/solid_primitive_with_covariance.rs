use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SolidPrimitiveWithCovariance {
    pub r#type: u8,
    pub dimensions: Vec<f64>,
    pub covariance: Vec<f64>,
}

impl SolidPrimitiveWithCovariance {
    pub const BOX: u8 = 1;
    pub const SPHERE: u8 = 2;
    pub const CYLINDER: u8 = 3;
    pub const CONE: u8 = 4;
    pub const BOX_X: u8 = 0;
    pub const BOX_Y: u8 = 1;
    pub const BOX_Z: u8 = 2;
    pub const SPHERE_RADIUS: u8 = 0;
    pub const CYLINDER_HEIGHT: u8 = 0;
    pub const CYLINDER_RADIUS: u8 = 1;
    pub const CONE_HEIGHT: u8 = 0;
    pub const CONE_RADIUS: u8 = 1;
}

impl Default for SolidPrimitiveWithCovariance {
    fn default() -> Self {
        SolidPrimitiveWithCovariance {
            r#type: 0,
            dimensions: Vec::new(),
            covariance: Vec::new(),
        }
    }
}
