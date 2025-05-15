use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BehaviorParameter {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for BehaviorParameter {
    fn default() -> Self {
        BehaviorParameter {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}
