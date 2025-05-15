use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChainConnection {
    pub name: ::std::string::String,
    pub reference_interfaces: Vec<::std::string::String>,
}

impl Default for ChainConnection {
    fn default() -> Self {
        ChainConnection {
            name: ::std::string::String::new(),
            reference_interfaces: Vec::new(),
        }
    }
}
