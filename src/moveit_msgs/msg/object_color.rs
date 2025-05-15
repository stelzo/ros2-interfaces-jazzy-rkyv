use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectColor {
    pub id: ::std::string::String,
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for ObjectColor {
    fn default() -> Self {
        ObjectColor {
            id: ::std::string::String::new(),
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}
