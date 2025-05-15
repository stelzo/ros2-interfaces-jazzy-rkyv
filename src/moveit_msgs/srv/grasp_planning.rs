use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GraspPlanningRequest {
    pub group_name: ::std::string::String,
    pub target: crate::moveit_msgs::msg::CollisionObject,
    pub support_surfaces: Vec<::std::string::String>,
    pub candidate_grasps: Vec<crate::moveit_msgs::msg::Grasp>,
    pub movable_obstacles: Vec<crate::moveit_msgs::msg::CollisionObject>,
}

impl Default for GraspPlanningRequest {
    fn default() -> Self {
        GraspPlanningRequest {
            group_name: ::std::string::String::new(),
            target: crate::moveit_msgs::msg::CollisionObject::default(),
            support_surfaces: Vec::new(),
            candidate_grasps: Vec::new(),
            movable_obstacles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GraspPlanningResponse {
    pub grasps: Vec<crate::moveit_msgs::msg::Grasp>,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GraspPlanningResponse {
    fn default() -> Self {
        GraspPlanningResponse {
            grasps: Vec::new(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

pub struct GraspPlanning;
