use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LaneletMapBin {
    pub header: crate::std_msgs::msg::Header,
    pub version_map_format: ::std::string::String,
    pub version_map: ::std::string::String,
    pub name_map: ::std::string::String,
    pub data: Vec<u8>,
}

impl Default for LaneletMapBin {
    fn default() -> Self {
        LaneletMapBin {
            header: crate::std_msgs::msg::Header::default(),
            version_map_format: ::std::string::String::new(),
            version_map: ::std::string::String::new(),
            name_map: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}
