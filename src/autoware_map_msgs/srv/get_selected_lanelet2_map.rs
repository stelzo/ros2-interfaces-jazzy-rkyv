use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetSelectedLanelet2MapRequest {
    pub cell_ids: Vec<::std::string::String>,
}

impl Default for GetSelectedLanelet2MapRequest {
    fn default() -> Self {
        GetSelectedLanelet2MapRequest {
            cell_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetSelectedLanelet2MapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub lanelet2_cells: crate::autoware_map_msgs::msg::LaneletMapBin,
}

impl Default for GetSelectedLanelet2MapResponse {
    fn default() -> Self {
        GetSelectedLanelet2MapResponse {
            header: crate::std_msgs::msg::Header::default(),
            lanelet2_cells: crate::autoware_map_msgs::msg::LaneletMapBin::default(),
        }
    }
}

pub struct GetSelectedLanelet2Map;
