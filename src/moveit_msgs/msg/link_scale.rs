use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LinkScale {
    pub link_name: ::std::string::String,
    pub scale: f64,
}

impl Default for LinkScale {
    fn default() -> Self {
        LinkScale {
            link_name: ::std::string::String::new(),
            scale: 0.0,
        }
    }
}
