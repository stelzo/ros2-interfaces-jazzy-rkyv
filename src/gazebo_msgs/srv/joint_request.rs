use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointRequestRequest {
    pub joint_name: ::std::string::String,
}

impl Default for JointRequestRequest {
    fn default() -> Self {
        JointRequestRequest {
            joint_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointRequestResponse {}

impl Default for JointRequestResponse {
    fn default() -> Self {
        JointRequestResponse {}
    }
}

pub struct JointRequest;
