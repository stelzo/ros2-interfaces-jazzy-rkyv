use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Joy {
    pub header: crate::std_msgs::msg::Header,
    pub axes: Vec<f32>,
    pub buttons: Vec<i32>,
}

impl Default for Joy {
    fn default() -> Self {
        Joy {
            header: crate::std_msgs::msg::Header::default(),
            axes: Vec::new(),
            buttons: Vec::new(),
        }
    }
}
