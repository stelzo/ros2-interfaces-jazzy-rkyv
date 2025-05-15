use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRegionsOfInterest2DRequest {
    pub region_of_interest_2d_ids: Vec<::std::string::String>,
}

impl Default for GetRegionsOfInterest2DRequest {
    fn default() -> Self {
        GetRegionsOfInterest2DRequest {
            region_of_interest_2d_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRegionsOfInterest2DResponse {
    pub regions_of_interest: Vec<crate::rc_reason_msgs::msg::RegionOfInterest2D>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetRegionsOfInterest2DResponse {
    fn default() -> Self {
        GetRegionsOfInterest2DResponse {
            regions_of_interest: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct GetRegionsOfInterest2D;
