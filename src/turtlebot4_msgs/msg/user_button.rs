use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UserButton {
    pub button: [bool; 4],
}

impl Default for UserButton {
    fn default() -> Self {
        UserButton { button: [false; 4] }
    }
}
