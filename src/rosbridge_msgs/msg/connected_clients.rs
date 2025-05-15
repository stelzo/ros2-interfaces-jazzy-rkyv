use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConnectedClients {
    pub clients: Vec<crate::rosbridge_msgs::msg::ConnectedClient>,
}

impl Default for ConnectedClients {
    fn default() -> Self {
        ConnectedClients {
            clients: Vec::new(),
        }
    }
}
