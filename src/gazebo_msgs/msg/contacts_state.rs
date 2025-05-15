use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ContactsState {
    pub header: crate::std_msgs::msg::Header,
    pub states: Vec<crate::gazebo_msgs::msg::ContactState>,
}

impl Default for ContactsState {
    fn default() -> Self {
        ContactsState {
            header: crate::std_msgs::msg::Header::default(),
            states: Vec::new(),
        }
    }
}
