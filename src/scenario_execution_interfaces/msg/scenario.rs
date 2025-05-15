use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Scenario {
    pub name: ::std::string::String,
    pub scenario_file: ::std::string::String,
}

impl Default for Scenario {
    fn default() -> Self {
        Scenario {
            name: ::std::string::String::new(),
            scenario_file: ::std::string::String::new(),
        }
    }
}
