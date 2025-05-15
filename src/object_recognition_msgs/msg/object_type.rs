use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectType {
    pub key: ::std::string::String,
    pub db: ::std::string::String,
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType {
            key: ::std::string::String::new(),
            db: ::std::string::String::new(),
        }
    }
}
