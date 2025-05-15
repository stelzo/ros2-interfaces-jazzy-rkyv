use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadRequest {
    pub filename: ::std::string::String,
}

impl Default for LoadRequest {
    fn default() -> Self {
        LoadRequest {
            filename: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadResponse {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for LoadResponse {
    fn default() -> Self {
        LoadResponse {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct Load;
