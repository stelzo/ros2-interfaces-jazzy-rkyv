use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRegionOfInterest3DRequest {
    pub region_of_interest: crate::rc_reason_msgs::msg::RegionOfInterest3D,
}

impl Default for SetRegionOfInterest3DRequest {
    fn default() -> Self {
        SetRegionOfInterest3DRequest {
            region_of_interest: crate::rc_reason_msgs::msg::RegionOfInterest3D::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRegionOfInterest3DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetRegionOfInterest3DResponse {
    fn default() -> Self {
        SetRegionOfInterest3DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct SetRegionOfInterest3D;
