use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MeshFile {
    pub filename: ::std::string::String,
    pub data: Vec<u8>,
}

impl Default for MeshFile {
    fn default() -> Self {
        MeshFile {
            filename: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}
