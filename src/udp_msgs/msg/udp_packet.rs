use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UdpPacket {
    pub header: crate::std_msgs::msg::Header,
    pub address: ::std::string::String,
    pub src_port: u16,
    pub data: Vec<u8>,
}

impl Default for UdpPacket {
    fn default() -> Self {
        UdpPacket {
            header: crate::std_msgs::msg::Header::default(),
            address: ::std::string::String::new(),
            src_port: 0,
            data: Vec::new(),
        }
    }
}
