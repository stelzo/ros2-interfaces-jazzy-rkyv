use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TypeSource {
    pub type_name: ::std::string::String,
    pub encoding: ::std::string::String,
    pub raw_file_contents: ::std::string::String,
}

impl Default for TypeSource {
    fn default() -> Self {
        TypeSource {
            type_name: ::std::string::String::new(),
            encoding: ::std::string::String::new(),
            raw_file_contents: ::std::string::String::new(),
        }
    }
}
