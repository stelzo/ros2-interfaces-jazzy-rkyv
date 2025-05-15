use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UniversalJointPos {
    pub name: ::std::string::String,
    pub ax1: f32,
    pub ax2: f32,
}

impl Default for UniversalJointPos {
    fn default() -> Self {
        UniversalJointPos {
            name: ::std::string::String::new(),
            ax1: 0.0,
            ax2: 0.0,
        }
    }
}
