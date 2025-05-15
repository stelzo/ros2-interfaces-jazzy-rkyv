use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FieldInformation {
    pub field_id: u8,
    pub field_set_id: u8,
    pub field_result: u8,
    pub eval_method: u8,
    pub field_active: u8,
}

impl Default for FieldInformation {
    fn default() -> Self {
        FieldInformation {
            field_id: 0,
            field_set_id: 0,
            field_result: 0,
            eval_method: 0,
            field_active: 0,
        }
    }
}
