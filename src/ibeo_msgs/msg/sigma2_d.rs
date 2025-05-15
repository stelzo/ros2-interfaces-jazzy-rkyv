use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Sigma2D {
    pub sigma_x: u16,
    pub sigma_y: u16,
}

impl Default for Sigma2D {
    fn default() -> Self {
        Sigma2D {
            sigma_x: 0,
            sigma_y: 0,
        }
    }
}
