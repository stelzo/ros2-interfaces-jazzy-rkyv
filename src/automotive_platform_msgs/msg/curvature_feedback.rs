use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CurvatureFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub curvature: f32,
}

impl Default for CurvatureFeedback {
    fn default() -> Self {
        CurvatureFeedback {
            header: crate::std_msgs::msg::Header::default(),
            curvature: 0.0,
        }
    }
}
