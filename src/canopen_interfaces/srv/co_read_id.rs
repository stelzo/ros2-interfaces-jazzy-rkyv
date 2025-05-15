use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COReadIDRequest {
    pub nodeid: u8,
    pub index: u16,
    pub subindex: u8,
    pub canopen_datatype: u8,
}

impl COReadIDRequest {
    pub const CANOPEN_DATATYPE_INT8: u8 = 0x02;
    pub const CANOPEN_DATATYPE_INT16: u8 = 0x03;
    pub const CANOPEN_DATATYPE_INT32: u8 = 0x04;
    pub const CANOPEN_DATATYPE_UINT8: u8 = 0x05;
    pub const CANOPEN_DATATYPE_UINT16: u8 = 0x06;
    pub const CANOPEN_DATATYPE_UINT32: u8 = 0x07;
}

impl Default for COReadIDRequest {
    fn default() -> Self {
        COReadIDRequest {
            nodeid: 0,
            index: 0,
            subindex: 0,
            canopen_datatype: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COReadIDResponse {
    pub success: bool,
    pub data: u32,
}

impl Default for COReadIDResponse {
    fn default() -> Self {
        COReadIDResponse {
            success: false,
            data: 0,
        }
    }
}

pub struct COReadID;
