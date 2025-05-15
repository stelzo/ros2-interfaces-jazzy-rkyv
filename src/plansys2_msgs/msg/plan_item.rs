use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PlanItem {
    pub time: f32,
    pub action: ::std::string::String,
    pub duration: f32,
}

impl Default for PlanItem {
    fn default() -> Self {
        PlanItem {
            time: 0.0,
            action: ::std::string::String::new(),
            duration: 0.0,
        }
    }
}
