use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GlobalDescriptor {
    pub header: crate::std_msgs::msg::Header,
    pub r#type: i32,
    pub info: Vec<u8>,
    pub data: Vec<u8>,
}

impl Default for GlobalDescriptor {
    fn default() -> Self {
        GlobalDescriptor {
            header: crate::std_msgs::msg::Header::default(),
            r#type: 0,
            info: Vec::new(),
            data: Vec::new(),
        }
    }
}
