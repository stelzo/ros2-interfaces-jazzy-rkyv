use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UserData {
    pub header: crate::std_msgs::msg::Header,
    pub rows: u32,
    pub cols: u32,
    pub r#type: u32,
    pub data: Vec<u8>,
}

impl Default for UserData {
    fn default() -> Self {
        UserData {
            header: crate::std_msgs::msg::Header::default(),
            rows: 0,
            cols: 0,
            r#type: 0,
            data: Vec::new(),
        }
    }
}
