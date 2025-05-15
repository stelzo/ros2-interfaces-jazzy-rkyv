use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LinkPadding {
    pub link_name: ::std::string::String,
    pub padding: f64,
}

impl Default for LinkPadding {
    fn default() -> Self {
        LinkPadding {
            link_name: ::std::string::String::new(),
            padding: 0.0,
        }
    }
}
