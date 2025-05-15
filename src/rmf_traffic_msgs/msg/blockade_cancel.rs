use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BlockadeCancel {
    pub participant: u64,
    pub all_reservations: bool,
    pub reservation: u64,
}

impl Default for BlockadeCancel {
    fn default() -> Self {
        BlockadeCancel {
            participant: 0,
            all_reservations: false,
            reservation: 0,
        }
    }
}
