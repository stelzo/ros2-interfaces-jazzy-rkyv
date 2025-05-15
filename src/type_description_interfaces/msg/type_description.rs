use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TypeDescription {
    pub type_description: crate::type_description_interfaces::msg::IndividualTypeDescription,
    pub referenced_type_descriptions:
        Vec<crate::type_description_interfaces::msg::IndividualTypeDescription>,
}

impl Default for TypeDescription {
    fn default() -> Self {
        TypeDescription {
            type_description:
                crate::type_description_interfaces::msg::IndividualTypeDescription::default(),
            referenced_type_descriptions: Vec::new(),
        }
    }
}
