use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RelocalizeFromGNSSRequest {}

impl Default for RelocalizeFromGNSSRequest {
    fn default() -> Self {
        RelocalizeFromGNSSRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RelocalizeFromGNSSResponse {
    pub accepted: bool,
}

impl Default for RelocalizeFromGNSSResponse {
    fn default() -> Self {
        RelocalizeFromGNSSResponse { accepted: false }
    }
}

pub struct RelocalizeFromGNSS;
