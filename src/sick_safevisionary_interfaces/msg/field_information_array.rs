use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FieldInformationArray {
    pub header: crate::std_msgs::msg::Header,
    pub fields: Vec<crate::sick_safevisionary_interfaces::msg::FieldInformation>,
}

impl Default for FieldInformationArray {
    fn default() -> Self {
        FieldInformationArray {
            header: crate::std_msgs::msg::Header::default(),
            fields: Vec::new(),
        }
    }
}
