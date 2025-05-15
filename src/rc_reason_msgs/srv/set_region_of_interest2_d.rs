use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRegionOfInterest2DRequest {
    pub region_of_interest_2d: crate::rc_reason_msgs::msg::RegionOfInterest2D,
}

impl Default for SetRegionOfInterest2DRequest {
    fn default() -> Self {
        SetRegionOfInterest2DRequest {
            region_of_interest_2d: crate::rc_reason_msgs::msg::RegionOfInterest2D::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRegionOfInterest2DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetRegionOfInterest2DResponse {
    fn default() -> Self {
        SetRegionOfInterest2DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct SetRegionOfInterest2D;
