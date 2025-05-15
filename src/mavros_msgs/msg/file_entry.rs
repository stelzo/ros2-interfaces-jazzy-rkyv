use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FileEntry {
    pub name: ::std::string::String,
    pub r#type: u8,
    pub size: u64,
}

impl FileEntry {
    pub const TYPE_FILE: u8 = 0;
    pub const TYPE_DIRECTORY: u8 = 1;
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: ::std::string::String::new(),
            r#type: 0,
            size: 0,
        }
    }
}
