use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectHypothesis {
    pub class_id: ::std::string::String,
    pub score: f64,
}

impl Default for ObjectHypothesis {
    fn default() -> Self {
        ObjectHypothesis {
            class_id: ::std::string::String::new(),
            score: 0.0,
        }
    }
}
