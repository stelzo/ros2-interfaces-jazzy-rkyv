use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadDatabaseRequest {
    pub database_path: ::std::string::String,
    pub clear: bool,
}

impl Default for LoadDatabaseRequest {
    fn default() -> Self {
        LoadDatabaseRequest {
            database_path: ::std::string::String::new(),
            clear: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadDatabaseResponse {}

impl Default for LoadDatabaseResponse {
    fn default() -> Self {
        LoadDatabaseResponse {}
    }
}

pub struct LoadDatabase;
