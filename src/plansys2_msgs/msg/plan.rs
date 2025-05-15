use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Plan {
    pub items: Vec<crate::plansys2_msgs::msg::PlanItem>,
}

impl Default for Plan {
    fn default() -> Self {
        Plan { items: Vec::new() }
    }
}
