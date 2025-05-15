use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AudioNoteVector {
    pub header: crate::std_msgs::msg::Header,
    pub notes: Vec<crate::irobot_create_msgs::msg::AudioNote>,
    pub append: bool,
}

impl Default for AudioNoteVector {
    fn default() -> Self {
        AudioNoteVector {
            header: crate::std_msgs::msg::Header::default(),
            notes: Vec::new(),
            append: false,
        }
    }
}
