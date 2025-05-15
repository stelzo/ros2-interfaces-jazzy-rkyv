use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApplyPlanningSceneRequest {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for ApplyPlanningSceneRequest {
    fn default() -> Self {
        ApplyPlanningSceneRequest {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApplyPlanningSceneResponse {
    pub success: bool,
}

impl Default for ApplyPlanningSceneResponse {
    fn default() -> Self {
        ApplyPlanningSceneResponse { success: false }
    }
}

pub struct ApplyPlanningScene;
