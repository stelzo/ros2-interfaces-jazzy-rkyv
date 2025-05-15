use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SerializedGraph {
    pub header: crate::std_msgs::msg::Header,
    pub plugin_name: ::std::string::String,
    pub data: Vec<u8>,
}

impl Default for SerializedGraph {
    fn default() -> Self {
        SerializedGraph {
            header: crate::std_msgs::msg::Header::default(),
            plugin_name: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}
