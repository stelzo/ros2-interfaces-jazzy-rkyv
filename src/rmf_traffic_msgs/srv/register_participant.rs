use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RegisterParticipantRequest {
    pub description: crate::rmf_traffic_msgs::msg::ParticipantDescription,
}

impl Default for RegisterParticipantRequest {
    fn default() -> Self {
        RegisterParticipantRequest {
            description: crate::rmf_traffic_msgs::msg::ParticipantDescription::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RegisterParticipantResponse {
    pub participant_id: u64,
    pub last_itinerary_version: u64,
    pub last_plan_id: u64,
    pub next_storage_base: u64,
    pub error: ::std::string::String,
}

impl Default for RegisterParticipantResponse {
    fn default() -> Self {
        RegisterParticipantResponse {
            participant_id: 0,
            last_itinerary_version: 0,
            last_plan_id: 0,
            next_storage_base: 0,
            error: ::std::string::String::new(),
        }
    }
}

pub struct RegisterParticipant;
