use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddLinkRequest {
    pub link: crate::rtabmap_msgs::msg::Link,
}

impl Default for AddLinkRequest {
    fn default() -> Self {
        AddLinkRequest {
            link: crate::rtabmap_msgs::msg::Link::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddLinkResponse {}

impl Default for AddLinkResponse {
    fn default() -> Self {
        AddLinkResponse {}
    }
}

pub struct AddLink;
