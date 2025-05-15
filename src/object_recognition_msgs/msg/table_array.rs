use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TableArray {
    pub header: crate::std_msgs::msg::Header,
    pub tables: Vec<crate::object_recognition_msgs::msg::Table>,
}

impl Default for TableArray {
    fn default() -> Self {
        TableArray {
            header: crate::std_msgs::msg::Header::default(),
            tables: Vec::new(),
        }
    }
}
