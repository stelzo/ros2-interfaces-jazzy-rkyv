use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UserdataInfo {
    pub state: ::std::string::String,
    pub key: ::std::string::String,
    pub r#type: ::std::string::String,
    pub data: ::std::string::String,
}

impl Default for UserdataInfo {
    fn default() -> Self {
        UserdataInfo {
            state: ::std::string::String::new(),
            key: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            data: ::std::string::String::new(),
        }
    }
}
