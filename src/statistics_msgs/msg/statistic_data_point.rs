use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StatisticDataPoint {
    pub data_type: u8,
    pub data: f64,
}

impl Default for StatisticDataPoint {
    fn default() -> Self {
        StatisticDataPoint {
            data_type: 0,
            data: 0.0,
        }
    }
}
