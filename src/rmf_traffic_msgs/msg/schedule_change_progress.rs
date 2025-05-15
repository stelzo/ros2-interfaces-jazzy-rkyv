use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleChangeProgress {
    pub has_progress: bool,
    pub version: u64,
    pub checkpoints: Vec<u64>,
}

impl Default for ScheduleChangeProgress {
    fn default() -> Self {
        ScheduleChangeProgress {
            has_progress: false,
            version: 0,
            checkpoints: Vec::new(),
        }
    }
}
