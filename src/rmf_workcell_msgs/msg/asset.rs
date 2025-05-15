use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Asset {
    pub guid: ::std::string::String,
    pub r#type: ::std::string::String,
}

impl Default for Asset {
    fn default() -> Self {
        Asset {
            guid: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
        }
    }
}
