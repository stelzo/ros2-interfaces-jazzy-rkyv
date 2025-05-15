use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JamStateCentFreq {
    pub cent_freq: u32,
    pub jammed: bool,
}

impl Default for JamStateCentFreq {
    fn default() -> Self {
        JamStateCentFreq {
            cent_freq: 0,
            jammed: false,
        }
    }
}
