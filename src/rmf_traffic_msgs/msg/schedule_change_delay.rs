use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleChangeDelay {
    pub delay: i64,
}

impl Default for ScheduleChangeDelay {
    fn default() -> Self {
        ScheduleChangeDelay { delay: 0 }
    }
}
