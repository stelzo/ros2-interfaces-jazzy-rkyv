use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleQueryParticipants {
    pub r#type: u16,
    pub ids: Vec<u64>,
}

impl ScheduleQueryParticipants {
    pub const ALL: u16 = 1;
    pub const INCLUDE: u16 = 2;
    pub const EXCLUDE: u16 = 3;
}

impl Default for ScheduleQueryParticipants {
    fn default() -> Self {
        ScheduleQueryParticipants {
            r#type: 0,
            ids: Vec::new(),
        }
    }
}
