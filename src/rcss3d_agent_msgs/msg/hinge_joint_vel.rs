use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HingeJointVel {
    pub name: ::std::string::String,
    pub ax: f32,
}

impl Default for HingeJointVel {
    fn default() -> Self {
        HingeJointVel {
            name: ::std::string::String::new(),
            ax: 0.0,
        }
    }
}
