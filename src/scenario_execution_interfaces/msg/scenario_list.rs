use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScenarioList {
    pub scenarios: Vec<crate::scenario_execution_interfaces::msg::Scenario>,
}

impl Default for ScenarioList {
    fn default() -> Self {
        ScenarioList {
            scenarios: Vec::new(),
        }
    }
}
