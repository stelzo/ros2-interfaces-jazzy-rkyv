use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LaneletPrimitive {
    pub id: i64,
    pub primitive_type: ::std::string::String,
}

impl Default for LaneletPrimitive {
    fn default() -> Self {
        LaneletPrimitive {
            id: 0,
            primitive_type: ::std::string::String::new(),
        }
    }
}
