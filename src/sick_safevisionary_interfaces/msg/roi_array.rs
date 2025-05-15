use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ROIArray {
    pub header: crate::std_msgs::msg::Header,
    pub rois: Vec<crate::sick_safevisionary_interfaces::msg::ROI>,
}

impl Default for ROIArray {
    fn default() -> Self {
        ROIArray {
            header: crate::std_msgs::msg::Header::default(),
            rois: Vec::new(),
        }
    }
}
