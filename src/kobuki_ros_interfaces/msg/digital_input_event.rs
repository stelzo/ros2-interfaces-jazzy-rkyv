use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DigitalInputEvent {
    pub values: [bool; 4],
}

impl Default for DigitalInputEvent {
    fn default() -> Self {
        DigitalInputEvent { values: [false; 4] }
    }
}
