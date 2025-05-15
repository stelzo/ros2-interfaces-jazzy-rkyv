use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeliveryAlertAction {
    pub value: u32,
}

impl DeliveryAlertAction {
    pub const WAITING: u32 = 0;
    pub const CANCEL: u32 = 1;
    pub const OVERRIDE: u32 = 2;
    pub const RESUME: u32 = 3;
}

impl Default for DeliveryAlertAction {
    fn default() -> Self {
        DeliveryAlertAction { value: 0 }
    }
}
