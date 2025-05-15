use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Dictionary {
    pub dictionary_uuid: u32,
    pub names: Vec<::std::string::String>,
}

impl Default for Dictionary {
    fn default() -> Self {
        Dictionary {
            dictionary_uuid: 0,
            names: Vec::new(),
        }
    }
}
