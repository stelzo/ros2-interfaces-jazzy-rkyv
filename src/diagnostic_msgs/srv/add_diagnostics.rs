use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddDiagnosticsRequest {
    pub load_namespace: ::std::string::String,
}

impl Default for AddDiagnosticsRequest {
    fn default() -> Self {
        AddDiagnosticsRequest {
            load_namespace: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddDiagnosticsResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for AddDiagnosticsResponse {
    fn default() -> Self {
        AddDiagnosticsResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct AddDiagnostics;
