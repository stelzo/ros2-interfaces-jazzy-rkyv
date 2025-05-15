use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Property {
    pub property: ::std::string::String,
    pub anything: ::std::string::String,
}

impl Default for Property {
    fn default() -> Self {
        Property {
            property: ::std::string::String::new(),
            anything: ::std::string::String::new(),
        }
    }
}
