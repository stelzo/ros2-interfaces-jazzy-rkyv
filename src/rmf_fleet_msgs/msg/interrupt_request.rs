use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct InterruptRequest {
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub interrupt_id: ::std::string::String,
    pub labels: Vec<::std::string::String>,
    pub r#type: u8,
}

impl InterruptRequest {
    pub const TYPE_INTERRUPT: u8 = 0;
    pub const TYPE_RESUME: u8 = 1;
}

impl Default for InterruptRequest {
    fn default() -> Self {
        InterruptRequest {
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            interrupt_id: ::std::string::String::new(),
            labels: Vec::new(),
            r#type: 0,
        }
    }
}
