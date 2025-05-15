use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NovatelExtendedSolutionStatus {
    pub original_mask: u32,
    pub advance_rtk_verified: bool,
    pub psuedorange_iono_correction: ::std::string::String,
}

impl Default for NovatelExtendedSolutionStatus {
    fn default() -> Self {
        NovatelExtendedSolutionStatus {
            original_mask: 0,
            advance_rtk_verified: false,
            psuedorange_iono_correction: ::std::string::String::new(),
        }
    }
}
