use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLightPropertiesRequest {
    pub light_name: ::std::string::String,
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
}

impl Default for SetLightPropertiesRequest {
    fn default() -> Self {
        SetLightPropertiesRequest {
            light_name: ::std::string::String::new(),
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            attenuation_constant: 0.0,
            attenuation_linear: 0.0,
            attenuation_quadratic: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLightPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLightPropertiesResponse {
    fn default() -> Self {
        SetLightPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetLightProperties;
