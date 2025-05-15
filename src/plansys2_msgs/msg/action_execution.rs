use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActionExecution {
    pub r#type: i16,
    pub node_id: ::std::string::String,
    pub action: ::std::string::String,
    pub arguments: Vec<::std::string::String>,
    pub success: bool,
    pub completion: f32,
    pub status: ::std::string::String,
}

impl ActionExecution {
    pub const REQUEST: i16 = 1;
    pub const RESPONSE: i16 = 2;
    pub const CONFIRM: i16 = 3;
    pub const REJECT: i16 = 4;
    pub const FEEDBACK: i16 = 5;
    pub const FINISH: i16 = 6;
    pub const CANCEL: i16 = 7;
}

impl Default for ActionExecution {
    fn default() -> Self {
        ActionExecution {
            r#type: 0,
            node_id: ::std::string::String::new(),
            action: ::std::string::String::new(),
            arguments: Vec::new(),
            success: false,
            completion: 0.0,
            status: ::std::string::String::new(),
        }
    }
}
