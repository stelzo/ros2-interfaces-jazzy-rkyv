use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SynthesisErrorCodes {
    pub value: i32,
}

impl SynthesisErrorCodes {
    pub const SUCCESS: i32 = 1;
}

impl Default for SynthesisErrorCodes {
    fn default() -> Self {
        SynthesisErrorCodes { value: 0 }
    }
}
