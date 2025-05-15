use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Derived {
    pub predicate: crate::plansys2_msgs::msg::Node,
    pub preconditions: crate::plansys2_msgs::msg::Tree,
}

impl Default for Derived {
    fn default() -> Self {
        Derived {
            predicate: crate::plansys2_msgs::msg::Node::default(),
            preconditions: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}
