use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReassignCommandIdCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub command_id_index: u8,
    pub user_command_id: u32,
    pub disable_default_command_id: bool,
}

impl Default for ReassignCommandIdCmd {
    fn default() -> Self {
        ReassignCommandIdCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            command_id_index: 0,
            user_command_id: 0,
            disable_default_command_id: false,
        }
    }
}
