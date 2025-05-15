use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UdpSendRequest {
    pub local_address: ::std::string::String,
    pub local_port: u16,
    pub remote_address: ::std::string::String,
    pub remote_port: u16,
    pub data: Vec<u8>,
}

impl Default for UdpSendRequest {
    fn default() -> Self {
        UdpSendRequest {
            local_address: ::std::string::String::new(),
            local_port: 0,
            remote_address: ::std::string::String::new(),
            remote_port: 0,
            data: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UdpSendResponse {
    pub sent: bool,
}

impl Default for UdpSendResponse {
    fn default() -> Self {
        UdpSendResponse { sent: false }
    }
}

pub struct UdpSend;
