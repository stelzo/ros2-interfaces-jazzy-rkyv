use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FieldSetReadSrvRequest {}

impl Default for FieldSetReadSrvRequest {
    fn default() -> Self {
        FieldSetReadSrvRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FieldSetReadSrvResponse {
    pub field_set_selection_method: i32,
    pub active_field_set: i32,
    pub success: bool,
}

impl Default for FieldSetReadSrvResponse {
    fn default() -> Self {
        FieldSetReadSrvResponse {
            field_set_selection_method: 0,
            active_field_set: 0,
            success: false,
        }
    }
}

pub struct FieldSetReadSrv;
