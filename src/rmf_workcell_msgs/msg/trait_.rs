use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Trait {
    pub key: ::std::string::String,
    pub value: Vec<::std::string::String>,
}

impl Default for Trait {
    fn default() -> Self {
        Trait {
            key: ::std::string::String::new(),
            value: Vec::new(),
        }
    }
}
