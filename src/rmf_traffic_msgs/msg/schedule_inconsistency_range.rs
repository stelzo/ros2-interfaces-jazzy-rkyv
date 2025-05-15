use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleInconsistencyRange {
    pub lower: u64,
    pub upper: u64,
}

impl Default for ScheduleInconsistencyRange {
    fn default() -> Self {
        ScheduleInconsistencyRange { lower: 0, upper: 0 }
    }
}
