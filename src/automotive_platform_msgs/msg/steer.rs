use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Steer {
    pub header: crate::std_msgs::msg::Header,
    pub module_name: ::std::string::String,
    pub curvature: f32,
    pub max_curvature_rate: f32,
}

impl Default for Steer {
    fn default() -> Self {
        Steer {
            header: crate::std_msgs::msg::Header::default(),
            module_name: ::std::string::String::new(),
            curvature: 0.0,
            max_curvature_rate: 0.0,
        }
    }
}
