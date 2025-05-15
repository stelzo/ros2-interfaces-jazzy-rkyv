use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccEventGenerator {
    pub index: i32,
    pub type_name: ::std::string::String,
    pub object_tag: ::std::string::String,
}

impl Default for SmaccEventGenerator {
    fn default() -> Self {
        SmaccEventGenerator {
            index: 0,
            type_name: ::std::string::String::new(),
            object_tag: ::std::string::String::new(),
        }
    }
}
