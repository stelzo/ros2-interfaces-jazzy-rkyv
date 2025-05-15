use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SonarUsage {
    pub left: bool,
    pub right: bool,
}

impl Default for SonarUsage {
    fn default() -> Self {
        SonarUsage {
            left: false,
            right: false,
        }
    }
}
