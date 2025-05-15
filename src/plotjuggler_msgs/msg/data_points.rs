use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DataPoints {
    pub dictionary_uuid: u32,
    pub samples: Vec<crate::plotjuggler_msgs::msg::DataPoint>,
}

impl Default for DataPoints {
    fn default() -> Self {
        DataPoints {
            dictionary_uuid: 0,
            samples: Vec::new(),
        }
    }
}
