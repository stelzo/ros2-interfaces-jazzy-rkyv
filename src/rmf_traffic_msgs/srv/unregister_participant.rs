use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UnregisterParticipantRequest {
    pub participant_id: u64,
}

impl Default for UnregisterParticipantRequest {
    fn default() -> Self {
        UnregisterParticipantRequest { participant_id: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UnregisterParticipantResponse {
    pub confirmation: bool,
    pub error: ::std::string::String,
}

impl Default for UnregisterParticipantResponse {
    fn default() -> Self {
        UnregisterParticipantResponse {
            confirmation: false,
            error: ::std::string::String::new(),
        }
    }
}

pub struct UnregisterParticipant;
