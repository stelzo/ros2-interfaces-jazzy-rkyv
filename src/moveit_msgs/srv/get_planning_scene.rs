use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlanningSceneRequest {
    pub components: crate::moveit_msgs::msg::PlanningSceneComponents,
}

impl Default for GetPlanningSceneRequest {
    fn default() -> Self {
        GetPlanningSceneRequest {
            components: crate::moveit_msgs::msg::PlanningSceneComponents::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlanningSceneResponse {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for GetPlanningSceneResponse {
    fn default() -> Self {
        GetPlanningSceneResponse {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

pub struct GetPlanningScene;
