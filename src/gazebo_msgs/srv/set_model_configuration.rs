use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetModelConfigurationRequest {
    pub model_name: ::std::string::String,
    pub urdf_param_name: ::std::string::String,
    pub joint_names: Vec<::std::string::String>,
    pub joint_positions: Vec<f64>,
}

impl Default for SetModelConfigurationRequest {
    fn default() -> Self {
        SetModelConfigurationRequest {
            model_name: ::std::string::String::new(),
            urdf_param_name: ::std::string::String::new(),
            joint_names: Vec::new(),
            joint_positions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetModelConfigurationResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetModelConfigurationResponse {
    fn default() -> Self {
        SetModelConfigurationResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetModelConfiguration;
