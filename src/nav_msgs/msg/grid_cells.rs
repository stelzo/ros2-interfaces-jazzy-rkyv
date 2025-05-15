use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GridCells {
    pub header: crate::std_msgs::msg::Header,
    pub cell_width: f32,
    pub cell_height: f32,
    pub cells: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for GridCells {
    fn default() -> Self {
        GridCells {
            header: crate::std_msgs::msg::Header::default(),
            cell_width: 0.0,
            cell_height: 0.0,
            cells: Vec::new(),
        }
    }
}
