use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Flag {
    pub name: ::std::string::String,
    pub base: crate::rcss3d_agent_msgs::msg::Spherical,
}

impl Default for Flag {
    fn default() -> Self {
        Flag {
            name: ::std::string::String::new(),
            base: crate::rcss3d_agent_msgs::msg::Spherical::default(),
        }
    }
}
