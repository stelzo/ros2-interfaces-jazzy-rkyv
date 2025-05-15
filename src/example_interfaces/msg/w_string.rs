use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WString {
    pub data: ::std::string::String,
}

impl Default for WString {
    fn default() -> Self {
        WString {
            data: ::std::string::String::new(),
        }
    }
}
