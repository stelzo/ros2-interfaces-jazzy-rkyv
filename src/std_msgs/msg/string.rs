use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct String {
    pub data: ::std::string::String,
}

impl Default for String {
    fn default() -> Self {
        String {
            data: ::std::string::String::new(),
        }
    }
}
