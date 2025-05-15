use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Link {
    pub from_id: i32,
    pub to_id: i32,
    pub r#type: i32,
    pub transform: crate::geometry_msgs::msg::Transform,
    pub information: [f64; 36],
}

impl Default for Link {
    fn default() -> Self {
        Link {
            from_id: 0,
            to_id: 0,
            r#type: 0,
            transform: crate::geometry_msgs::msg::Transform::default(),
            information: [0.0; 36],
        }
    }
}
