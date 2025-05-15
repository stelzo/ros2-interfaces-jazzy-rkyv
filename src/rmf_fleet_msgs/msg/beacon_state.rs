use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BeaconState {
    pub id: ::std::string::String,
    pub online: bool,
    pub category: ::std::string::String,
    pub activated: bool,
    pub level: ::std::string::String,
}

impl Default for BeaconState {
    fn default() -> Self {
        BeaconState {
            id: ::std::string::String::new(),
            online: false,
            category: ::std::string::String::new(),
            activated: false,
            level: ::std::string::String::new(),
        }
    }
}
