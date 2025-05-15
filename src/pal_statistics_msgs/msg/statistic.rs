use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Statistic {
    pub name: ::std::string::String,
    pub value: f64,
}

impl Default for Statistic {
    fn default() -> Self {
        Statistic {
            name: ::std::string::String::new(),
            value: 0.0,
        }
    }
}
