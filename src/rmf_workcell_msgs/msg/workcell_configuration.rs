use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WorkcellConfiguration {
    pub time: crate::builtin_interfaces::msg::Time,
    pub guid: ::std::string::String,
    pub r#type: ::std::string::String,
    pub assets: Vec<crate::rmf_workcell_msgs::msg::Asset>,
    pub traits: Vec<crate::rmf_workcell_msgs::msg::Trait>,
}

impl Default for WorkcellConfiguration {
    fn default() -> Self {
        WorkcellConfiguration {
            time: crate::builtin_interfaces::msg::Time::default(),
            guid: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            assets: Vec::new(),
            traits: Vec::new(),
        }
    }
}
