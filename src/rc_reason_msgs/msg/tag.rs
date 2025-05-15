use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Tag {
    pub id: ::std::string::String,
    pub size: f64,
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: ::std::string::String::new(),
            size: 0.0,
        }
    }
}
