use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PointCloudMapCellMetaDataWithID {
    pub cell_id: ::std::string::String,
    pub metadata: crate::autoware_map_msgs::msg::PointCloudMapCellMetaData,
}

impl Default for PointCloudMapCellMetaDataWithID {
    fn default() -> Self {
        PointCloudMapCellMetaDataWithID {
            cell_id: ::std::string::String::new(),
            metadata: crate::autoware_map_msgs::msg::PointCloudMapCellMetaData::default(),
        }
    }
}
