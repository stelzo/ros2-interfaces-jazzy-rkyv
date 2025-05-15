use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ItineraryDelay {
    pub participant: u64,
    pub delay: i64,
    pub itinerary_version: u64,
}

impl Default for ItineraryDelay {
    fn default() -> Self {
        ItineraryDelay {
            participant: 0,
            delay: 0,
            itinerary_version: 0,
        }
    }
}
