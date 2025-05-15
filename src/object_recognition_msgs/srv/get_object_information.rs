use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetObjectInformationRequest {
    pub r#type: crate::object_recognition_msgs::msg::ObjectType,
}

impl Default for GetObjectInformationRequest {
    fn default() -> Self {
        GetObjectInformationRequest {
            r#type: crate::object_recognition_msgs::msg::ObjectType::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetObjectInformationResponse {
    pub information: crate::object_recognition_msgs::msg::ObjectInformation,
}

impl Default for GetObjectInformationResponse {
    fn default() -> Self {
        GetObjectInformationResponse {
            information: crate::object_recognition_msgs::msg::ObjectInformation::default(),
        }
    }
}

pub struct GetObjectInformation;
