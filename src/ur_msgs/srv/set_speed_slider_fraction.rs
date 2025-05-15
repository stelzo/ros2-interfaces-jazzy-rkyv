use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetSpeedSliderFractionRequest {
    pub speed_slider_fraction: f64,
}

impl Default for SetSpeedSliderFractionRequest {
    fn default() -> Self {
        SetSpeedSliderFractionRequest {
            speed_slider_fraction: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetSpeedSliderFractionResponse {
    pub success: bool,
}

impl Default for SetSpeedSliderFractionResponse {
    fn default() -> Self {
        SetSpeedSliderFractionResponse { success: false }
    }
}

pub struct SetSpeedSliderFraction;
