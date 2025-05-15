use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SoundRequest {
    pub value: u8,
}

impl Default for SoundRequest {
    fn default() -> Self {
        SoundRequest { value: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SoundResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SoundResponse {
    fn default() -> Self {
        SoundResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct Sound;
