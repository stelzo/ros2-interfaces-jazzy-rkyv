use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleChangeCull {
    pub time: i64,
}

impl Default for ScheduleChangeCull {
    fn default() -> Self {
        ScheduleChangeCull { time: 0 }
    }
}
