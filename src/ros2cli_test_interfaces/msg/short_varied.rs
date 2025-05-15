use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ShortVaried {
    pub bool_value: bool,
    pub bool_values: Vec<bool>,
}

impl ShortVaried {
    pub const BOOL_CONST: bool = true;
}

impl Default for ShortVaried {
    fn default() -> Self {
        ShortVaried {
            bool_value: false,
            bool_values: Vec::new(),
        }
    }
}
