use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Goalpost {
    pub bb: crate::vision_msgs::msg::BoundingBox3D,
    pub attributes: crate::soccer_vision_attribute_msgs::msg::Goalpost,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for Goalpost {
    fn default() -> Self {
        Goalpost {
            bb: crate::vision_msgs::msg::BoundingBox3D::default(),
            attributes: crate::soccer_vision_attribute_msgs::msg::Goalpost::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}
