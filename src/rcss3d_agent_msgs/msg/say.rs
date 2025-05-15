use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Say {
    pub message: ::std::string::String,
}

impl Default for Say {
    fn default() -> Self {
        Say {
            message: ::std::string::String::new(),
        }
    }
}
