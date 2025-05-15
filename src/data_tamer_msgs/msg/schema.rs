use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Schema {
    pub hash: u64,
    pub channel_name: ::std::string::String,
    pub schema_text: ::std::string::String,
}

impl Default for Schema {
    fn default() -> Self {
        Schema {
            hash: 0,
            channel_name: ::std::string::String::new(),
            schema_text: ::std::string::String::new(),
        }
    }
}
