use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ExecuteScenarioRequest {
    pub scenario: crate::scenario_execution_interfaces::msg::Scenario,
}

impl Default for ExecuteScenarioRequest {
    fn default() -> Self {
        ExecuteScenarioRequest {
            scenario: crate::scenario_execution_interfaces::msg::Scenario::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ExecuteScenarioResponse {
    pub result: bool,
}

impl Default for ExecuteScenarioResponse {
    fn default() -> Self {
        ExecuteScenarioResponse { result: false }
    }
}

pub struct ExecuteScenario;
