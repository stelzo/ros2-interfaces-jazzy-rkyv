use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Contacts {
    pub header: crate::std_msgs::msg::Header,
    pub contacts: Vec<crate::ros_gz_interfaces::msg::Contact>,
}

impl Default for Contacts {
    fn default() -> Self {
        Contacts {
            header: crate::std_msgs::msg::Header::default(),
            contacts: Vec::new(),
        }
    }
}
