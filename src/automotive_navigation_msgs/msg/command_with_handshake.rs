use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandWithHandshake {
    pub header: crate::std_msgs::msg::Header,
    pub msg_counter: u8,
    pub command: i16,
}

impl Default for CommandWithHandshake {
    fn default() -> Self {
        CommandWithHandshake {
            header: crate::std_msgs::msg::Header::default(),
            msg_counter: 0,
            command: 0,
        }
    }
}
