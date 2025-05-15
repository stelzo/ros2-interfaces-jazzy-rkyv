use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UserDisplay {
    pub ip: ::std::string::String,
    pub battery: ::std::string::String,
    pub entries: [::std::string::String; 5],
    pub selected_entry: i32,
}

impl Default for UserDisplay {
    fn default() -> Self {
        UserDisplay {
            ip: ::std::string::String::new(),
            battery: ::std::string::String::new(),
            entries: core::array::from_fn(|_| ::std::string::String::default()),
            selected_entry: 0,
        }
    }
}
