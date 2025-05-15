use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DescribeParametersRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for DescribeParametersRequest {
    fn default() -> Self {
        DescribeParametersRequest { names: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DescribeParametersResponse {
    pub descriptors: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
}

impl Default for DescribeParametersResponse {
    fn default() -> Self {
        DescribeParametersResponse {
            descriptors: Vec::new(),
        }
    }
}

pub struct DescribeParameters;
