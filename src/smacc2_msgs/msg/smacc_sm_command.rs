use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccSMCommand {
    pub command: i8,
}

impl SmaccSMCommand {
    pub const SM_STOP: i8 = 0;
    pub const E_STOP: i8 = 0;
    pub const SM_RESET: i8 = 1;
}

impl Default for SmaccSMCommand {
    fn default() -> Self {
        SmaccSMCommand { command: 0 }
    }
}
