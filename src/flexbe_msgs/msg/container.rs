use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Container {
    pub state_id: i32,
    pub path: ::std::string::String,
    pub children: Vec<::std::string::String>,
    pub outcomes: Vec<::std::string::String>,
    pub transitions: Vec<::std::string::String>,
    pub r#type: i8,
    pub autonomy: Vec<i8>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            state_id: 0,
            path: ::std::string::String::new(),
            children: Vec::new(),
            outcomes: Vec::new(),
            transitions: Vec::new(),
            r#type: 0,
            autonomy: Vec::new(),
        }
    }
}
