use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UBXNavClock {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub clk_b: i32,
    pub clk_d: i32,
    pub t_acc: u32,
    pub f_acc: u32,
}

impl Default for UBXNavClock {
    fn default() -> Self {
        UBXNavClock {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            clk_b: 0,
            clk_d: 0,
            t_acc: 0,
            f_acc: 0,
        }
    }
}
