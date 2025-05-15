use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PopupRequest {
    pub message: ::std::string::String,
}

impl Default for PopupRequest {
    fn default() -> Self {
        PopupRequest {
            message: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PopupResponse {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for PopupResponse {
    fn default() -> Self {
        PopupResponse {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct Popup;
