use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MonVERExtension {
    pub field: [i8; 30],
}

impl Default for MonVERExtension {
    fn default() -> Self {
        MonVERExtension { field: [0; 30] }
    }
}
