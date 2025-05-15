use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRegionsOfInterest2DRequest {
    pub region_of_interest_2d_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest2DRequest {
    fn default() -> Self {
        DeleteRegionsOfInterest2DRequest {
            region_of_interest_2d_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRegionsOfInterest2DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest2DResponse {
    fn default() -> Self {
        DeleteRegionsOfInterest2DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct DeleteRegionsOfInterest2D;
