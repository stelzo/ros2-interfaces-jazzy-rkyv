use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRegionsOfInterest3DRequest {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for GetRegionsOfInterest3DRequest {
    fn default() -> Self {
        GetRegionsOfInterest3DRequest {
            region_of_interest_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRegionsOfInterest3DResponse {
    pub regions_of_interest: Vec<crate::rc_reason_msgs::msg::RegionOfInterest3D>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetRegionsOfInterest3DResponse {
    fn default() -> Self {
        GetRegionsOfInterest3DResponse {
            regions_of_interest: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct GetRegionsOfInterest3D;
