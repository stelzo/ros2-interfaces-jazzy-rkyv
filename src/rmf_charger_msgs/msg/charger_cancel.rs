use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChargerCancel {
    pub charger_name: ::std::string::String,
    pub request_id: ::std::string::String,
}

impl Default for ChargerCancel {
    fn default() -> Self {
        ChargerCancel {
            charger_name: ::std::string::String::new(),
            request_id: ::std::string::String::new(),
        }
    }
}
