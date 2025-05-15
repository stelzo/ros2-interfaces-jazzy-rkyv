use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AffectParamRequest {
    pub param: crate::plansys2_msgs::msg::Param,
}

impl Default for AffectParamRequest {
    fn default() -> Self {
        AffectParamRequest {
            param: crate::plansys2_msgs::msg::Param::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AffectParamResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectParamResponse {
    fn default() -> Self {
        AffectParamResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct AffectParam;
