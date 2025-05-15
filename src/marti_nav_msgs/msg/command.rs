use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Command {
    pub header: crate::std_msgs::msg::Header,
    pub startstop: Vec<i32>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            header: crate::std_msgs::msg::Header::default(),
            startstop: Vec::new(),
        }
    }
}
