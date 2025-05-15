use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WorldReset {
    pub all: bool,        // default: false
    pub time_only: bool,  // default: false
    pub model_only: bool, // default: false
}

impl Default for WorldReset {
    fn default() -> Self {
        WorldReset {
            all: false,
            time_only: false,
            model_only: false,
        }
    }
}
