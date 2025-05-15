use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestMultipleRequestFieldsRequest {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleRequestFieldsRequest {
    fn default() -> Self {
        TestMultipleRequestFieldsRequest {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestMultipleRequestFieldsResponse {}

impl Default for TestMultipleRequestFieldsResponse {
    fn default() -> Self {
        TestMultipleRequestFieldsResponse {}
    }
}

pub struct TestMultipleRequestFields;
