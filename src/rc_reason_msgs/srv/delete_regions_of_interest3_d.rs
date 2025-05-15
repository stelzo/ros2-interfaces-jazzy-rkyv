use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRegionsOfInterest3DRequest {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest3DRequest {
    fn default() -> Self {
        DeleteRegionsOfInterest3DRequest {
            region_of_interest_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRegionsOfInterest3DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest3DResponse {
    fn default() -> Self {
        DeleteRegionsOfInterest3DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct DeleteRegionsOfInterest3D;
