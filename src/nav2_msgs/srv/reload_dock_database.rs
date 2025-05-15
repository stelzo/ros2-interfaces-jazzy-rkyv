use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReloadDockDatabaseRequest {
    pub filepath: ::std::string::String,
}

impl Default for ReloadDockDatabaseRequest {
    fn default() -> Self {
        ReloadDockDatabaseRequest {
            filepath: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReloadDockDatabaseResponse {
    pub success: bool,
}

impl Default for ReloadDockDatabaseResponse {
    fn default() -> Self {
        ReloadDockDatabaseResponse { success: false }
    }
}

pub struct ReloadDockDatabase;
