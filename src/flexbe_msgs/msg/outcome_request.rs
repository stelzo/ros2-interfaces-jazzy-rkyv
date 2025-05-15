use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OutcomeRequest {
    pub outcome: u8,
    pub target: ::std::string::String,
}

impl Default for OutcomeRequest {
    fn default() -> Self {
        OutcomeRequest {
            outcome: 0,
            target: ::std::string::String::new(),
        }
    }
}
