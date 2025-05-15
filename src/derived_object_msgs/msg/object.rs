use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Object {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub detection_level: u8,
    pub object_classified: bool,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub accel: crate::geometry_msgs::msg::Accel,
    pub polygon: crate::geometry_msgs::msg::Polygon,
    pub shape: crate::shape_msgs::msg::SolidPrimitive,
    pub classification: u8,
    pub classification_certainty: u8,
    pub classification_age: u32,
}

impl Object {
    pub const OBJECT_DETECTED: u8 = 0;
    pub const OBJECT_TRACKED: u8 = 1;
    pub const CLASSIFICATION_UNKNOWN: u8 = 0;
    pub const CLASSIFICATION_UNKNOWN_SMALL: u8 = 1;
    pub const CLASSIFICATION_UNKNOWN_MEDIUM: u8 = 2;
    pub const CLASSIFICATION_UNKNOWN_BIG: u8 = 3;
    pub const CLASSIFICATION_PEDESTRIAN: u8 = 4;
    pub const CLASSIFICATION_BIKE: u8 = 5;
    pub const CLASSIFICATION_CAR: u8 = 6;
    pub const CLASSIFICATION_TRUCK: u8 = 7;
    pub const CLASSIFICATION_MOTORCYCLE: u8 = 8;
    pub const CLASSIFICATION_OTHER_VEHICLE: u8 = 9;
    pub const CLASSIFICATION_BARRIER: u8 = 10;
    pub const CLASSIFICATION_SIGN: u8 = 11;
}

impl Default for Object {
    fn default() -> Self {
        Object {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            detection_level: 0,
            object_classified: false,
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            accel: crate::geometry_msgs::msg::Accel::default(),
            polygon: crate::geometry_msgs::msg::Polygon::default(),
            shape: crate::shape_msgs::msg::SolidPrimitive::default(),
            classification: 0,
            classification_certainty: 0,
            classification_age: 0,
        }
    }
}
