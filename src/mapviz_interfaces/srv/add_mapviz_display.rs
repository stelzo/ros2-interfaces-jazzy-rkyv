use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddMapvizDisplayRequest {
    pub name: ::std::string::String,
    pub r#type: ::std::string::String,
    pub draw_order: i32,
    pub visible: bool,
    pub properties: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for AddMapvizDisplayRequest {
    fn default() -> Self {
        AddMapvizDisplayRequest {
            name: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            draw_order: 0,
            visible: false,
            properties: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddMapvizDisplayResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for AddMapvizDisplayResponse {
    fn default() -> Self {
        AddMapvizDisplayResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct AddMapvizDisplay;
