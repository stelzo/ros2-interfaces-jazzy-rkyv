use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CameraParameter {
    pub name: ::std::string::String,
    pub value: f64,
}

impl Default for CameraParameter {
    fn default() -> Self {
        CameraParameter {
            name: ::std::string::String::new(),
            value: 0.0,
        }
    }
}
