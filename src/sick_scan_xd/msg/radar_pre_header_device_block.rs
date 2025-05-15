use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RadarPreHeaderDeviceBlock {
    pub uiident: u32,
    pub udiserialno: u32,
    pub bdeviceerror: bool,
    pub bcontaminationwarning: bool,
    pub bcontaminationerror: bool,
}

impl Default for RadarPreHeaderDeviceBlock {
    fn default() -> Self {
        RadarPreHeaderDeviceBlock {
            uiident: 0,
            udiserialno: 0,
            bdeviceerror: false,
            bcontaminationwarning: false,
            bcontaminationerror: false,
        }
    }
}
