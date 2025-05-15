use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RadarPreHeaderEncoderBlock {
    pub udiencoderpos: u32,
    pub iencoderspeed: i16,
}

impl Default for RadarPreHeaderEncoderBlock {
    fn default() -> Self {
        RadarPreHeaderEncoderBlock {
            udiencoderpos: 0,
            iencoderspeed: 0,
        }
    }
}
