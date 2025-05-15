use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Action {
    pub name: ::std::string::String,
    pub parameters: Vec<crate::plansys2_msgs::msg::Param>,
    pub preconditions: crate::plansys2_msgs::msg::Tree,
    pub effects: crate::plansys2_msgs::msg::Tree,
}

impl Default for Action {
    fn default() -> Self {
        Action {
            name: ::std::string::String::new(),
            parameters: Vec::new(),
            preconditions: crate::plansys2_msgs::msg::Tree::default(),
            effects: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}
