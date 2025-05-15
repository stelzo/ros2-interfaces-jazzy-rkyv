use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Mode {
    pub label: ::std::string::String,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            label: ::std::string::String::new(),
        }
    }
}
