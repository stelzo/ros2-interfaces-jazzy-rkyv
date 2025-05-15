use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Hear {
    pub team: ::std::string::String,
    pub time: f32,
    pub self_: bool,
    pub direction: Vec<f32>,
    pub message: ::std::string::String,
}

impl Default for Hear {
    fn default() -> Self {
        Hear {
            team: ::std::string::String::new(),
            time: 0.0,
            self_: false,
            direction: Vec::new(),
            message: ::std::string::String::new(),
        }
    }
}
