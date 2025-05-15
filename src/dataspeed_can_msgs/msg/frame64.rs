use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Frame64 {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub extended: bool,
    pub fdf: bool,
    pub brs: bool,
    pub esi: bool,
    pub rtr: bool,
    pub size: u8,
    pub data: [u8; 64],
}

impl Default for Frame64 {
    fn default() -> Self {
        Frame64 {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            extended: false,
            fdf: false,
            brs: false,
            esi: false,
            rtr: false,
            size: 0,
            data: [0; 64],
        }
    }
}
