use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeviceInfoRequest {}

impl Default for DeviceInfoRequest {
    fn default() -> Self {
        DeviceInfoRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeviceInfoResponse {
    pub device_name: ::std::string::String,
    pub serial_number: ::std::string::String,
    pub firmware_version: ::std::string::String,
    pub usb_type_descriptor: ::std::string::String,
    pub firmware_update_id: ::std::string::String,
    pub sensors: ::std::string::String,
    pub physical_port: ::std::string::String,
}

impl Default for DeviceInfoResponse {
    fn default() -> Self {
        DeviceInfoResponse {
            device_name: ::std::string::String::new(),
            serial_number: ::std::string::String::new(),
            firmware_version: ::std::string::String::new(),
            usb_type_descriptor: ::std::string::String::new(),
            firmware_update_id: ::std::string::String::new(),
            sensors: ::std::string::String::new(),
            physical_port: ::std::string::String::new(),
        }
    }
}

pub struct DeviceInfo;
