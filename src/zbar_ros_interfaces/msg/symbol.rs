use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Symbol {
    pub data: ::std::string::String,
    pub points: Vec<crate::vision_msgs::msg::Point2D>,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol {
            data: ::std::string::String::new(),
            points: Vec::new(),
        }
    }
}
