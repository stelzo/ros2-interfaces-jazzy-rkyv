use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReloadControllerLibrariesRequest {
    pub force_kill: bool,
}

impl Default for ReloadControllerLibrariesRequest {
    fn default() -> Self {
        ReloadControllerLibrariesRequest { force_kill: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReloadControllerLibrariesResponse {
    pub ok: bool,
}

impl Default for ReloadControllerLibrariesResponse {
    fn default() -> Self {
        ReloadControllerLibrariesResponse { ok: false }
    }
}

pub struct ReloadControllerLibraries;
