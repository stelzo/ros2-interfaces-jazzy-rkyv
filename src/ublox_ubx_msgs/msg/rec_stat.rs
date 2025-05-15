use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RecStat {
    pub leap_sec: bool,
    pub clk_reset: bool,
}

impl Default for RecStat {
    fn default() -> Self {
        RecStat {
            leap_sec: false,
            clk_reset: false,
        }
    }
}
