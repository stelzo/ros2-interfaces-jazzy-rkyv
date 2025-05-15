use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WarmupTemplateRequest {
    pub template_id: ::std::string::String,
}

impl Default for WarmupTemplateRequest {
    fn default() -> Self {
        WarmupTemplateRequest {
            template_id: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WarmupTemplateResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for WarmupTemplateResponse {
    fn default() -> Self {
        WarmupTemplateResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct WarmupTemplate;
