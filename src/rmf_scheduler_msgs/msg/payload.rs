use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Payload {
    pub r#type: u8,
    pub topic: ::std::string::String,
    pub message_type: ::std::string::String,
    pub data: Vec<u8>,
}

impl Payload {
    pub const PAYLOAD_TYPE_SERIALIZED_MESSAGE: u8 = 1;
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            r#type: 0,
            topic: ::std::string::String::new(),
            message_type: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}
