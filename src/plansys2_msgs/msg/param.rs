use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Param {
    pub name: ::std::string::String,
    pub r#type: ::std::string::String,
    pub sub_types: Vec<::std::string::String>,
}

impl Default for Param {
    fn default() -> Self {
        Param {
            name: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            sub_types: Vec::new(),
        }
    }
}
