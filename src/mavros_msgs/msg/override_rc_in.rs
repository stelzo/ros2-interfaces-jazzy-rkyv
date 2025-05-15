use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OverrideRCIn {
    pub channels: [u16; 18],
}

impl OverrideRCIn {
    pub const CHAN_RELEASE: u16 = 0;
    pub const CHAN_NOCHANGE: u16 = 65535;
}

impl Default for OverrideRCIn {
    fn default() -> Self {
        OverrideRCIn { channels: [0; 18] }
    }
}
