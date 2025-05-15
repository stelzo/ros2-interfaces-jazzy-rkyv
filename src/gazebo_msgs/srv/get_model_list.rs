use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModelListRequest {}

impl Default for GetModelListRequest {
    fn default() -> Self {
        GetModelListRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModelListResponse {
    pub header: crate::std_msgs::msg::Header,
    pub model_names: Vec<::std::string::String>,
    pub success: bool,
}

impl Default for GetModelListResponse {
    fn default() -> Self {
        GetModelListResponse {
            header: crate::std_msgs::msg::Header::default(),
            model_names: Vec::new(),
            success: false,
        }
    }
}

pub struct GetModelList;
