use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestMultipleResponseFieldsRequest {}

impl Default for TestMultipleResponseFieldsRequest {
    fn default() -> Self {
        TestMultipleResponseFieldsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestMultipleResponseFieldsResponse {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleResponseFieldsResponse {
    fn default() -> Self {
        TestMultipleResponseFieldsResponse {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

pub struct TestMultipleResponseFields;
