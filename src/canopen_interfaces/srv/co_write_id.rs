use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COWriteIDRequest {
    pub nodeid: i8,
    pub index: u16,
    pub subindex: u8,
    pub data: u32,
    pub canopen_datatype: u8,
}

impl COWriteIDRequest {
    pub const CANOPEN_DATATYPE_INT8: u8 = 0x02;
    pub const CANOPEN_DATATYPE_INT16: u8 = 0x03;
    pub const CANOPEN_DATATYPE_INT32: u8 = 0x04;
    pub const CANOPEN_DATATYPE_UINT8: u8 = 0x05;
    pub const CANOPEN_DATATYPE_UINT16: u8 = 0x06;
    pub const CANOPEN_DATATYPE_UINT32: u8 = 0x07;
}

impl Default for COWriteIDRequest {
    fn default() -> Self {
        COWriteIDRequest {
            nodeid: 0,
            index: 0,
            subindex: 0,
            data: 0,
            canopen_datatype: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COWriteIDResponse {
    pub success: bool,
}

impl Default for COWriteIDResponse {
    fn default() -> Self {
        COWriteIDResponse { success: false }
    }
}

pub struct COWriteID;
