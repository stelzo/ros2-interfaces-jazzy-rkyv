use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamValue {
    pub integer: i64,
    pub real: f64,
}

impl Default for ParamValue {
    fn default() -> Self {
        ParamValue {
            integer: 0,
            real: 0.0,
        }
    }
}
