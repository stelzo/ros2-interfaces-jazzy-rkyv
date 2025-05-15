use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CONmtIDRequest {
    pub nmtcommand: u8,
    pub nodeid: u8,
}

impl Default for CONmtIDRequest {
    fn default() -> Self {
        CONmtIDRequest {
            nmtcommand: 0,
            nodeid: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CONmtIDResponse {
    pub success: bool,
}

impl Default for CONmtIDResponse {
    fn default() -> Self {
        CONmtIDResponse { success: false }
    }
}

pub struct CONmtID;
